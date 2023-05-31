use std::fmt::{self};
use std::ptr;

use super::aadhaar::Aadhaar;
use super::dosasucana::Dosa;
use super::mulya::Mulya;

mod std_ank;
mod std_kul;
mod std_labhyate;
mod std_nirgam;
mod std_pathana;
mod std_prakaar;
mod std_samay;
mod std_shabd;
mod std_truti;
mod std_vad;

#[derive(Clone, Copy)]
pub struct MoolSutra(pub fn(&mut Aadhaar, usize) -> Result<Mulya, Dosa>);

impl MoolSutra {
    pub fn samay() -> Self {
        MoolSutra(std_samay::samay)
    }

    pub fn vad() -> Self {
        MoolSutra(std_vad::vad)
    }

    pub fn pathana() -> Self {
        MoolSutra(std_pathana::pathana)
    }

    pub fn nirgam() -> Self {
        MoolSutra(std_nirgam::nirgam)
    }

    pub fn labhyate() -> Self {
        MoolSutra(std_labhyate::labhyate)
    }

    pub fn prakaar() -> Self {
        MoolSutra(std_prakaar::prakaar)
    }

    pub fn kul() -> Self {
        MoolSutra(std_kul::kul)
    }

    pub fn truti() -> Self {
        MoolSutra(std_truti::truti)
    }

    pub fn shabd() -> Self {
        MoolSutra(std_shabd::shabd)
    }

    pub fn ank() -> Self {
        MoolSutra(std_ank::ank)
    }

    // pub fn for_each() -> Self {
    //     MoolSutra(std_list::for_each)
    // }

    // pub fn map() -> Self {
    //     MoolSutra(std_list::map)
    // }
}

impl fmt::Debug for MoolSutra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn>")
    }
}

impl PartialEq for MoolSutra {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}
