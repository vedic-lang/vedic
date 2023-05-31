use std::process;

use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function nirgam निर्गम : (int) -> exit
/// this function exits the program with the given exit code
pub fn nirgam(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    let code = match args.get(0) {
        Some(Mulya::Ank(code)) => *code as i32,
        _ => 0,
    };
    process::exit(code);
}
