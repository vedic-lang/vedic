use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Sourcecode<'sc> {
    pub path: &'sc str,
    pub code: String,
}

impl<'sc> Sourcecode<'sc> {
    pub fn new(filepath: &'sc str) -> io::Result<Sourcecode<'sc>> {
        // let abs_path = fs::canonicalize(filepath)?;
        let path = Path::new(filepath);
        let code = fs::read_to_string(path)?;
        Ok(Sourcecode {
            path: path.to_str().unwrap(),
            code,
        })
    }
}

pub fn import_path(source: &str, specifier: &str) -> Result<PathBuf, String> {
    let directory = Path::new(source).parent().unwrap_or(Path::new("."));
    match specifier.strip_prefix("pkg:") {
        #[cfg(not(target_arch = "wasm32"))]
        Some(name) => crate::packages::package_entry(directory, name),
        #[cfg(target_arch = "wasm32")]
        Some(_) => Err("Installed packages are available in the Vedic CLI".into()),
        None => Ok(directory.join(specifier)),
    }
}
