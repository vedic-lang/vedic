use std::fs;
use std::io::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Sourcecode<'sc> {
    pub path: &'sc str,
    pub code: String,
}

impl<'sc> Sourcecode<'sc> {
    pub fn new(filepath: &'sc str) -> Result<Sourcecode> {
        // let abs_path = fs::canonicalize(filepath)?;
        let path = Path::new(filepath);
        let code = fs::read_to_string(path)?;
        Ok(Sourcecode {
            path: path.to_str().unwrap(),
            code,
        })
    }
}
