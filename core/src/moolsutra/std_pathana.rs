use std::io;

use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function pathana पठन : () -> string
/// this function reads a line from stdin
pub fn pathana(aadhaar: &mut Aadhaar, _from: usize) -> Result<Mulya, Dosa> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to pathana line");

    let input = input.trim().to_string();
    Ok(Mulya::Vakya(aadhaar.gc.intern(input)))
}
