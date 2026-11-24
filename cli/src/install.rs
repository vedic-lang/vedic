use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::Command;

use fs2::FileExt;
use semver::Version;
use tempfile::NamedTempFile;
use tempfile::TempDir;
use toml_edit::value;
use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::Table;
use vedic_core::packages::check_name;
use vedic_core::packages::dependencies;
use vedic_core::packages::manifest_path;
use vedic_core::packages::read_manifest;
use vedic_core::packages::release_version;
use vedic_core::packages::reserved_filename;
use vedic_core::packages::store;

const INDEX_URL: &str = "https://github.com/vedic-lang/package-index.git";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run(args: &[String]) {
    if args.len() > 1 {
        eprintln!("Usage: vedic install [package-name]");
        process::exit(64);
    }
    let name = args.first().map(String::as_str);
    if let Some(error) = name.and_then(|n| check_name(n).err()) {
        eprintln!("{error}");
        process::exit(64);
    }
    if let Err(error) = install(name) {
        eprintln!("Install failed: {error}");
        process::exit(1);
    }
}

fn git(dir: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .current_dir(dir)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env("GIT_TERMINAL_PROMPT", "0")
        .args([
            "-c",
            "protocol.allow=never",
            "-c",
            "protocol.https.allow=always",
            "-c",
            "http.followRedirects=false",
        ])
        .args(args)
        .output()
        .map_err(|e| format!("Could not run Git: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {}: {}",
            args[0],
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }
    Ok(output.stdout)
}

fn valid_repository(repo: &str) -> bool {
    let Some((owner, name)) = repo.split_once('/') else {
        return false;
    };
    !owner.is_empty()
        && owner
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        && !name.is_empty()
        && !matches!(name, "." | "..")
        && !name.ends_with(".git")
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}

struct Listing {
    repo: String,
    releases: BTreeMap<Version, String>,
}

impl Listing {
    fn read(index: &Path, name: &str) -> Result<Self> {
        check_name(name)?;
        let path = format!("HEAD:packages/{name}.toml");
        let data = git(index, &["show", &path])
            .map_err(|_| format!("Package '{name}' is missing from the Vedic index"))?;
        let doc = std::str::from_utf8(&data)?.parse::<DocumentMut>()?;
        let repo = doc
            .get("repository")
            .and_then(Item::as_str)
            .filter(|s| valid_repository(s))
            .ok_or_else(|| format!("Invalid repository in the index entry for {name}"))?;
        let table = doc
            .get("releases")
            .and_then(Item::as_table)
            .ok_or("Missing [releases] in index entry")?;
        let mut releases = BTreeMap::new();
        for (key, item) in table {
            let sha = item
                .as_str()
                .filter(|s| {
                    s.len() == 40
                        && s.bytes()
                            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                })
                .ok_or_else(|| format!("Expected a full lowercase Git commit for {name} {key}"))?;
            releases.insert(release_version(key)?, sha.to_owned());
        }
        Ok(Self {
            repo: repo.into(),
            releases,
        })
    }
}

// Read regular blobs directly. Package hooks, checkout filters and submodules never run.
fn export_sources(repo: &Path, revision: &str, dest: &Path) -> Result<()> {
    let tree = git(repo, &["ls-tree", "-rz", revision])?;
    for row in tree.split(|&b| b == 0).filter(|r| !r.is_empty()) {
        let sep = row
            .iter()
            .position(|&b| b == b'\t')
            .ok_or("Invalid Git tree")?;
        let filename = std::str::from_utf8(&row[sep + 1..])?;
        let path = Path::new(filename);
        if path != Path::new("vedic.toml") && !crate::utilities::is_vedic_file(path) {
            continue;
        }
        let mut fields = std::str::from_utf8(&row[..sep])?.split(' ');
        if !matches!(fields.next(), Some("100644" | "100755")) || fields.next() != Some("blob") {
            return Err(format!("Expected a regular source file: {}", path.display()).into());
        }
        if filename.split('/').any(|part| {
            part.is_empty()
                || part.ends_with(['.', ' '])
                || reserved_filename(part)
                || part.contains(['\\', ':', '*', '?', '"', '<', '>', '|'])
                || part.chars().any(char::is_control)
        }) {
            return Err(format!("Unsupported package path: {filename}").into());
        }
        let output = dest.join(path);
        fs::create_dir_all(output.parent().ok_or("Invalid source path")?)?;
        let contents = git(
            repo,
            &["cat-file", "blob", fields.next().ok_or("Missing blob ID")?],
        )?;
        File::create_new(output)?.write_all(&contents)?;
    }
    Ok(())
}

fn verify_library(path: &Path, name: &str, version: &Version) -> Result<BTreeMap<String, Version>> {
    let doc = read_manifest(&path.join("vedic.toml"))?;
    let package = doc
        .get("package")
        .and_then(Item::as_table_like)
        .ok_or("Library needs a [package] table")?;
    if package.get("name").and_then(Item::as_str) != Some(name)
        || package.get("version").and_then(Item::as_str) != Some(version.to_string().as_str())
    {
        return Err(
            format!("Library manifest does not match indexed package {name} {version}").into(),
        );
    }
    if !path.join("lib.ved").is_file() {
        return Err(format!("Library {name} {version} has no lib.ved").into());
    }
    Ok(dependencies(&doc)?)
}

struct Installer {
    index: TempDir,
    root: PathBuf,
    installed: BTreeSet<(String, Version)>,
    visiting: BTreeSet<(String, Version)>,
}

impl Installer {
    fn new(root: PathBuf) -> Result<Self> {
        let index = TempDir::new()?;
        git(
            index.path(),
            &[
                "clone",
                "--bare",
                "--template=",
                "--depth=1",
                "--single-branch",
                "--branch",
                "main",
                "--",
                INDEX_URL,
                ".",
            ],
        )?;
        Ok(Self {
            index,
            root,
            installed: BTreeSet::new(),
            visiting: BTreeSet::new(),
        })
    }

    fn ensure(&mut self, name: &str, requested: Option<&Version>) -> Result<Version> {
        let listing = Listing::read(self.index.path(), name)?;
        let (version, revision) = match requested {
            Some(v) => listing.releases.get_key_value(v),
            None => listing.releases.last_key_value(),
        }
        .ok_or_else(|| {
            format!(
                "No indexed release of {name} matches {}",
                requested.map_or_else(|| "latest".into(), ToString::to_string)
            )
        })?;
        let key = (name.to_owned(), version.clone());
        if self.installed.contains(&key) {
            return Ok(version.clone());
        }
        if !self.visiting.insert(key.clone()) {
            return Err(format!("Circular package dependency at {name} {version}").into());
        }

        let parent = self.root.join(name);
        fs::create_dir_all(&parent)?;
        let destination = parent.join(version.to_string());
        let staging = if destination.try_exists()? {
            if fs::read_to_string(destination.join(".vedic-revision"))? != *revision {
                return Err(
                    format!("Cached revision differs from the index for {name} {version}").into(),
                );
            }
            None
        } else {
            let repo = TempDir::new()?;
            git(repo.path(), &["init", "--bare", "--template=", "--quiet"])?;
            let url = format!("https://github.com/{}.git", listing.repo);
            git(
                repo.path(),
                &["fetch", "--no-tags", "--depth=1", "--", &url, revision],
            )?;
            let fetched = git(
                repo.path(),
                &["rev-parse", "--verify", "FETCH_HEAD^{commit}"],
            )?;
            if std::str::from_utf8(&fetched)?.trim() != revision {
                return Err(
                    format!("Fetched commit differs from the index for {name} {version}").into(),
                );
            }
            let temp = TempDir::new_in(&parent)?;
            export_sources(repo.path(), revision, temp.path())?;
            fs::write(temp.path().join(".vedic-revision"), revision)?;
            Some(temp)
        };
        let source = staging.as_ref().map_or(destination.as_path(), |d| d.path());
        for (dep, pin) in verify_library(source, name, version)? {
            self.ensure(&dep, Some(&pin))?;
        }
        if let Some(temp) = staging {
            fs::rename(temp.path(), &destination)?;
        }
        self.visiting.remove(&key);
        self.installed.insert(key);
        Ok(version.clone())
    }
}

fn install(name: Option<&str>) -> Result<()> {
    let cwd = env::current_dir()?;
    let root = store()?;
    fs::create_dir_all(&root)?;
    let lock = File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(root.join(".install.lock"))?;
    lock.lock_exclusive()?;

    let project = manifest_path(&cwd)?;
    let mut doc = match &project {
        Some(path) => read_manifest(path)?,
        None if name.is_none() => {
            return Err("No vedic.toml found. Run 'vedic install <package-name>' first".into())
        }
        None => DocumentMut::new(),
    };
    let deps = dependencies(&doc)?;
    if name.is_none() && deps.is_empty() {
        println!("No dependencies to install.");
        return Ok(());
    }
    let mut installer = Installer::new(root)?;
    if let Some(name) = name {
        let version = installer.ensure(name, None)?;
        let path = project.unwrap_or_else(|| cwd.join("vedic.toml"));
        if deps.get(name) != Some(&version) {
            let table = doc
                .entry("dependencies")
                .or_insert(Item::Table(Table::new()))
                .as_table_like_mut()
                .unwrap();
            table.insert(name, value(version.to_string()));
            let mut output =
                NamedTempFile::new_in(path.parent().ok_or("Invalid project directory")?)?;
            if path.try_exists()? {
                if fs::symlink_metadata(&path)?.is_symlink() {
                    return Err(format!(
                        "Expected a regular project manifest at {}",
                        path.display()
                    )
                    .into());
                }
                output
                    .as_file()
                    .set_permissions(fs::metadata(&path)?.permissions())?;
            }
            output.write_all(doc.to_string().as_bytes())?;
            output.persist(&path)?;
        }
        println!("Installed {name} {version} for {}", path.display());
    } else {
        for (dep, version) in deps {
            installer.ensure(&dep, Some(&version))?;
            println!("Installed {dep} {version}");
        }
    }
    Ok(())
}
