use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function ank अंक : (string) -> int
/// this function converts a string to a number
pub fn ank(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    let arg = args.first();
    if let Some(arg) = arg {
        let mut num = String::new();
        let digit_mapping = ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'];
        for c in arg.to_string().chars() {
            let anukram = digit_mapping.iter().position(|&r| r == c);
            match anukram {
                Some(anukram) => num.push_str(&anukram.to_string()),
                None => num.push(c),
            }
        }
        if let Ok(ank) = num.parse::<f64>() {
            Ok(Mulya::Ank(ank))
        } else {
            Err(aadhaar.throw_dosa(
                ["वाक्य '", &num, "' को संख्या के रूप में परिवर्तित नहीं कर सकता"]
                    .join("")
                    .as_str(),
            ))
        }
    } else {
        Err(aadhaar.throw_dosa("Invalid argument to ank function"))
    }
}
