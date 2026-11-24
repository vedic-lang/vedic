//! Shared on-disk layout for installed native CLI packages.
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use semver::Version;
use toml_edit::DocumentMut;

// Windows reserves these names even with a file extension.
pub fn reserved_filename(component: &str) -> bool {
    let stem = component
        .split('.')
        .next()
        .unwrap_or("")
        .trim_end()
        .to_ascii_lowercase();
    matches!(
        stem.as_str(),
        "con" | "prn" | "aux" | "nul" | "conin$" | "conout$"
    ) || stem
        .strip_prefix("com")
        .or_else(|| stem.strip_prefix("lpt"))
        .is_some_and(|n| {
            matches!(
                n,
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
            )
        })
}

pub fn check_name(name: &str) -> Result<(), String> {
    let mut bytes = name.bytes();
    if name.len() <= 64
        && !reserved_filename(name)
        && bytes.next().is_some_and(|b| b.is_ascii_lowercase())
        && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        Ok(())
    } else {
        Err(format!(
            "Invalid package name '{name}'. Use a name from the Vedic package index."
        ))
    }
}

pub fn release_version(text: &str) -> Result<Version, String> {
    let version = Version::parse(text).map_err(|_| format!("Invalid package version '{text}'"))?;
    if !version.pre.is_empty() || !version.build.is_empty() {
        return Err(format!("Expected a stable package version, got '{text}'"));
    }
    Ok(version)
}

pub fn store() -> Result<PathBuf, String> {
    let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    let home = env::var_os("VEDIC_INSTALL")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| env::var_os(var).map(|s| Path::new(&s).join(".vedic")))
        .filter(|p| p.is_absolute())
        .ok_or("Set VEDIC_INSTALL to an absolute directory for your Vedic installation")?;
    Ok(home.join("packages"))
}

pub fn manifest_path(start: &Path) -> Result<Option<PathBuf>, String> {
    let start = env::current_dir().map_err(|e| e.to_string())?.join(start);
    for dir in start.ancestors() {
        let path = dir.join("vedic.toml");
        match fs::symlink_metadata(&path) {
            Ok(_) => return Ok(Some(path)),
            Err(e) if e.kind() == ErrorKind::NotFound => continue,
            Err(e) => return Err(format!("{}: {e}", path.display())),
        }
    }
    Ok(None)
}

pub fn read_manifest(path: &Path) -> Result<DocumentMut, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .parse()
        .map_err(|e| format!("{}: {e}", path.display()))
}

pub fn dependencies(doc: &DocumentMut) -> Result<BTreeMap<String, Version>, String> {
    let mut result = BTreeMap::new();
    if let Some(deps) = doc.get("dependencies") {
        let table = deps
            .as_table_like()
            .ok_or("Expected a [dependencies] table")?;
        for (name, value) in table.iter() {
            check_name(name)?;
            let version = value
                .as_str()
                .ok_or_else(|| format!("Expected a version string for {name}"))?;
            result.insert(name.into(), release_version(version)?);
        }
    }
    Ok(result)
}

pub fn package_entry(from: &Path, name: &str) -> Result<PathBuf, String> {
    check_name(name)?;
    let hint = format!("Run 'vedic install {name}' in your project");
    let manifest = manifest_path(from)?.ok_or_else(|| format!("No vedic.toml found. {hint}"))?;
    let deps = dependencies(&read_manifest(&manifest)?)?;
    let version = deps
        .get(name)
        .ok_or_else(|| format!("{name} is missing from {}. {hint}", manifest.display()))?;
    let entry = store()?
        .join(name)
        .join(version.to_string())
        .join("lib.ved");
    if !entry.is_file() {
        return Err(format!(
            "{name} {version} is missing from the user package store. Run 'vedic install' in your \
             project"
        ));
    }
    Ok(entry)
}
