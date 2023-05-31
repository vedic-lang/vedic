use super::super::aadhaar::Aadhaar;
use super::super::dosasucana::Dosa;
use super::super::mulya::Mulya;

/// function prakaar प्रकार : (any) -> string
/// this function returns the type of the given mulya
pub fn prakaar(aadhaar: &mut Aadhaar, from: usize) -> Result<Mulya, Dosa> {
    let args = &aadhaar.rashi[from..aadhaar.rashi_len()];

    if let Some(arg) = args.get(0) {
        match arg {
            Mulya::Vakya(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("वाक्य".to_owned()))),
            Mulya::Ank(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("अंक".to_owned()))),
            Mulya::Tarka(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("तर्क".to_owned()))),
            Mulya::Vidhi(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("विधि".to_owned()))),
            Mulya::Sutra(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("सूत्र".to_owned()))),
            Mulya::Avastha(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("उदाहरण".to_owned()))),
            Mulya::Paddhati(_v) => Ok(Mulya::Vakya(aadhaar.gc.intern("विधि सूत्र".to_owned()))),
            Mulya::Na => Ok(Mulya::Vakya(aadhaar.gc.intern("न".to_owned()))),
            _ => Ok(Mulya::Vakya(aadhaar.gc.intern("अज्ञात".to_owned()))),
        }
    } else {
        Err(aadhaar.throw_dosa("प्रकार सूत्र के लिए अमान्य मान दिए गए"))
    }
}
