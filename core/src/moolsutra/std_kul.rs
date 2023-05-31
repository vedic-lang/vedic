use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::type_of;
use super::super::mulya::Mulya;

/// function kul कुल : (string) -> int
/// this function returns the length of the given string
pub fn kul(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    let arg = args.get(0);
    if let Some(val) = arg {
        match val {
            Mulya::Vakya(string) => Ok(Mulya::Ank(string.s.len() as f64)),
            Mulya::Suchi(id) => Ok(Mulya::Ank(id.length() as f64)),
            val => {
                Err(aadhaar
                    .throw_dosa(format!("{:?} प्रकारः अपरिमिता नास्ति", type_of(val)).as_str()))
                // Ojbect of type {:?} has no length.
            }
        }
    } else {
        Err(aadhaar.throw_dosa("अमान्यः कार्यान्तर्गतस्य कुल परियन्त्रणस्य अभिकरणम्, वाक्यः अपेक्षितम्"))
        // Invalid argument to कुल function expected वाक्य
    }
}
