use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function labhyate लभ्यते : (string, substring) -> bool
/// this function returns true if the second string is a substring of the first
/// string
pub fn labhyate(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];
    if args.len() < 2 {
        return Err(aadhaar.throw_dosa("यत्र लभ्यते सूत्रे द्वे निरूपक वाक्यानि अपेक्षितानि"));
        // Expected 2 string arguments for लभ्यते function
    }
    if let Mulya::Vakya(string) = args[0] {
        if let Mulya::Vakya(needle) = args[1] {
            Ok(Mulya::Tarka(string.s.contains(needle.s.as_str())))
        } else {
            Err(aadhaar.throw_dosa("यत्र लभ्यते सूत्रे द्वे निरूपक वाक्यानि अपेक्षितानि"))
        }
    } else {
        Err(aadhaar.throw_dosa("यत्र लभ्यते सूत्रे द्वे निरूपक वाक्यानि अपेक्षितानि"))
    }
}
