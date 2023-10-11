use std::fmt;
use std::ops::Deref;

use super::moolsutra::MoolSutra;
use super::smrti_prabandhan::GcRef;
use super::vastuni::Avastha;
use super::vastuni::Paddhati;
use super::vastuni::Suchi;
use super::vastuni::Sutra;
use super::vastuni::Utsarga;
use super::vastuni::Vakya;
use super::vastuni::VastuType;
use super::vastuni::Vidhi;

#[derive(Clone, Copy, PartialEq)]
pub enum Mulya {
    Tarka(bool),
    Paddhati(GcRef<Paddhati>),
    Vidhi(GcRef<Vidhi>),
    Utsarga(GcRef<Utsarga>),
    Sutra(GcRef<Sutra>),
    Avastha(GcRef<Avastha>),
    MoolSutra(MoolSutra),
    Na,
    Anirdharita,
    Ank(f64),
    Vakya(GcRef<Vakya>),
    Suchi(GcRef<Suchi>),
}

impl Mulya {
    pub fn is_falsey(&self) -> bool {
        match self {
            Mulya::Na => true,
            Mulya::Tarka(mulya) => !mulya,
            Mulya::Ank(mulya) => *mulya == 0.0,
            Mulya::Vakya(mulya) => mulya.s.is_empty(),
            Mulya::Suchi(mulya) => mulya.is_empty(),
            _ => false,
        }
    }
}
pub fn type_of(mulya: &Mulya) -> VastuType {
    match mulya {
        Mulya::Avastha(_) => VastuType::Avastha,
        Mulya::Vidhi(_) => VastuType::Vidhi,
        Mulya::Paddhati(_) => VastuType::Paddhati,
        Mulya::Utsarga(_) => VastuType::Utsarga,
        Mulya::Sutra(_) => VastuType::Sutra,
        Mulya::MoolSutra(_) => VastuType::MoolSutra,
        Mulya::Tarka(_) => VastuType::Tarka,
        Mulya::Ank(_) => VastuType::Ank,
        Mulya::Vakya(_) => VastuType::Vakya,
        Mulya::Suchi(_) => VastuType::Suchi,
        Mulya::Na => VastuType::Na,
        Mulya::Anirdharita => VastuType::Anirdharita,
    }
}

impl fmt::Display for Mulya {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mulya::Tarka(mulya) => {
                write!(
                    f,
                    "{}",
                    if *mulya {
                        "सत्य"
                    } else {
                        "असत्य"
                    }
                )
            }
            Mulya::Paddhati(mulya) => write!(f, "{}", mulya.paddhati.sutra.deref()),
            Mulya::Vidhi(mulya) => write!(f, "{}", mulya.name.deref()),
            Mulya::Utsarga(mulya) => write!(f, "{}", mulya.sutra.deref()),
            Mulya::Sutra(mulya) => write!(f, "{}", mulya.name.deref()),
            Mulya::Avastha(mulya) => write!(f, "{} avastha", mulya.vidhi.name.deref()),
            Mulya::MoolSutra(_) => write!(f, "<mool fn>"), // TODO: print name
            Mulya::Na => write!(f, "न"),
            Mulya::Anirdharita => write!(f, "अनिर्धारित"),
            Mulya::Ank(mulya) => write!(f, "{}", cleaner(mulya)),
            Mulya::Vakya(mulya) => write!(f, "{}", mulya.deref()),
            Mulya::Suchi(mulya) => write!(f, "{}", mulya.deref()),
        }
    }
}

fn cleaner(num: &f64) -> String {
    let digit_mapping = ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'];
    let mut ank = String::new();
    let mulya = num.to_string();
    for c in mulya.as_bytes() {
        if c.is_ascii_digit() {
            let anukram = *c as usize - 48;
            ank.push(digit_mapping[anukram]);
        } else {
            ank.push(*c as char);
        }
    }
    ank
}
