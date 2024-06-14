use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function truti त्रुटि : (string) -> dosa
/// this function throws an dosa with the given string
pub fn truti(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    if let Some(arg) = args.first() {
        let dosa = format!("त्रुटि : {arg}");
        Err(aadhaar.throw_dosa(&dosa))
    } else {
        Err(aadhaar.throw_dosa("त्रुटि"))
    }
}
