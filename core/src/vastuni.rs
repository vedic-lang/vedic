use std::fmt;
use std::ops::Deref;

use super::mulya::Mulya;
use super::smrti::Smrti;
use super::smrti_prabandhan::GcRef;
use super::smrti_prabandhan::GcVastu;

#[derive(Clone, Copy, Debug)]
pub enum SutraType {
    Sutra,
    Mool,
    Paddhati,
    MoolLekha,
}

#[derive(Debug)]
pub enum VastuType {
    Sutra,
    Utsarga,
    Vakya,
    GyaatMaan,
    Vidhi,
    Avastha,
    Paddhati,
    Suchi,
    Tarka,
    Ank,
    Na,
    Anirdharita,
    MoolSutra,
}

#[repr(C)]
pub struct Suchi {
    pub mukhya: GcVastu,
    pub mulyas: Vec<Mulya>,
}

impl Suchi {
    pub fn new(mulyas: Vec<Mulya>) -> Self {
        Self {
            mukhya: GcVastu::new(VastuType::Suchi),
            mulyas,
        }
    }

    pub fn length(&self) -> usize {
        self.mulyas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    pub fn set(&mut self, anukram: usize, mulya: Mulya) {
        self.mulyas[anukram] = mulya;
    }
}

impl fmt::Display for Suchi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.mulyas
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
#[repr(C)]
pub struct Vakya {
    pub mukhya: GcVastu,
    pub s: String,
    pub hash: usize,
}

impl Vakya {
    pub fn from_string(s: String) -> Self {
        let hash = Vakya::hash_string(&s);
        Vakya {
            mukhya: GcVastu::new(VastuType::Vakya),
            s,
            hash,
        }
    }

    fn hash_string(s: &str) -> usize {
        let mut hash: usize = 2166136261;
        for b in s.bytes() {
            hash ^= b as usize;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }
}

impl fmt::Display for Vakya {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SutraGyaatMaan {
    pub anukram: u8,
    pub is_sthaniya: bool,
}

#[repr(C)]
pub struct Sutra {
    pub mukhya: GcVastu,
    pub arity: usize,
    pub vastu: Vastu,
    pub name: GcRef<Vakya>,
    pub kind: SutraType,
    pub gyaatmaans: Vec<SutraGyaatMaan>,
}

impl Sutra {
    pub fn new(name: GcRef<Vakya>, kind: SutraType) -> Self {
        Self {
            mukhya: GcVastu::new(VastuType::Sutra),
            arity: 0,
            vastu: Vastu::new(),
            name,
            kind,
            gyaatmaans: Vec::new(),
        }
    }
}

impl fmt::Display for Sutra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?} {}>", self.kind, self.name.deref())
    }
}

#[repr(C)]
pub struct GyaatMaan {
    pub mukhya: GcVastu,
    pub location: usize,
    pub closed: Option<Mulya>,
}

impl GyaatMaan {
    pub fn new(location: usize) -> Self {
        GyaatMaan {
            mukhya: GcVastu::new(VastuType::GyaatMaan),
            location,
            closed: None,
        }
    }
}

impl fmt::Display for GyaatMaan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gyaatmaan")
    }
}

#[repr(C)]
pub struct Utsarga {
    pub mukhya: GcVastu,
    pub sutra: GcRef<Sutra>,
    pub gyaatmaans: Vec<GcRef<GyaatMaan>>,
}

impl Utsarga {
    pub fn new(sutra: GcRef<Sutra>) -> Self {
        Utsarga {
            mukhya: GcVastu::new(VastuType::Utsarga),
            sutra,
            gyaatmaans: Vec::new(),
        }
    }
}

impl fmt::Display for Utsarga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sutra.deref())
    }
}

#[repr(C)]
pub struct Vidhi {
    pub mukhya: GcVastu,
    pub name: GcRef<Vakya>,
    pub paddhatis: Smrti,
}

impl Vidhi {
    pub fn new(name: GcRef<Vakya>) -> Self {
        Vidhi {
            mukhya: GcVastu::new(VastuType::Vidhi),
            name,
            paddhatis: Smrti::new(),
        }
    }
}

impl fmt::Display for Vidhi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.deref())
    }
}

#[repr(C)]
pub struct Avastha {
    pub mukhya: GcVastu,
    pub vidhi: GcRef<Vidhi>,
    pub fields: Smrti,
}

impl Avastha {
    pub fn new(vidhi: GcRef<Vidhi>) -> Self {
        Avastha {
            mukhya: GcVastu::new(VastuType::Avastha),
            vidhi,
            fields: Smrti::new(),
        }
    }
}

impl fmt::Display for Avastha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.vidhi.name.deref())
    }
}

#[repr(C)]
pub struct Paddhati {
    pub mukhya: GcVastu,
    pub adatr: Mulya,
    pub paddhati: GcRef<Utsarga>,
}

impl Paddhati {
    pub fn new(adatr: Mulya, paddhati: GcRef<Utsarga>) -> Self {
        Paddhati {
            mukhya: GcVastu::new(VastuType::Paddhati),
            adatr,
            paddhati,
        }
    }
}

impl fmt::Display for Paddhati {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.paddhati.sutra.deref())
    }
}

use super::aadesh::Aadesh;

pub struct Vastu {
    pub code: Vec<Aadesh>,
    pub achar: Vec<Mulya>,
    pub sthaan: Vec<String>,
}

impl Default for Vastu {
    fn default() -> Self {
        Self::new()
    }
}
impl Vastu {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            achar: Vec::new(),
            sthaan: Vec::new(),
        }
    }

    pub fn write(&mut self, instruction: Aadesh, sthaan: String) -> usize {
        self.code.push(instruction);
        self.sthaan.push(sthaan);
        self.code.len() - 1
    }

    pub fn pop(&mut self) {
        self.code.pop().unwrap();
        self.sthaan.pop().unwrap();
    }

    pub fn add_achar(&mut self, mulya: Mulya) -> usize {
        self.achar.push(mulya);
        self.achar.len() - 1
    }

    pub fn read_achar(&self, anukram: u8) -> Mulya {
        self.achar[anukram as usize]
    }

    pub fn read_vakya(&self, anukram: u8) -> GcRef<Vakya> {
        if let Mulya::Vakya(s) = self.read_achar(anukram) {
            s
        } else {
            panic!("एतद् अचर वाक्यं न अस्ति !")
        }
    }
}
