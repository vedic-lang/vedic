use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function shabd शब्द : (string) -> string
/// this function returns the given string
pub fn shabd(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    let arg = args.first();
    if let Some(arg) = arg {
        Ok(Mulya::Vakya(aadhaar.gc.intern(arg.to_string())))
    } else {
        Err(aadhaar.throw_dosa("Invalid argument to string Function"))
    }
}
