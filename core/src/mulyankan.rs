use super::pada::Pada;
use super::smrti_prabandhan::GcRef;
use super::vastuni::Sutra;
use super::vastuni::SutraGyaatMaan;
use super::vastuni::SutraType;
use super::vastuni::Vakya;

pub struct Mulyankan<'p> {
    pub enclosing: Option<Box<Mulyankan<'p>>>,
    pub sutra: Sutra,
    pub sthaniya: Vec<Sthaniya<'p>>,
    pub scope_depth: i32,
}

impl<'p> Mulyankan<'p> {
    pub const STHANIYA_COUNT: usize = u8::MAX as usize + 1;

    pub fn new(sutra_name: GcRef<Vakya>, kind: SutraType) -> Box<Self> {
        let mut evaluator = Mulyankan {
            enclosing: None,
            sutra: Sutra::new(sutra_name, kind),
            sthaniya: Vec::with_capacity(Mulyankan::STHANIYA_COUNT),
            scope_depth: 0,
        };

        let pada = match kind {
            SutraType::Paddhati | SutraType::Mool => Pada::default("मम"),
            _ => Pada::default(""),
        };
        evaluator.sthaniya.push(Sthaniya::new(pada, 0));
        Box::new(evaluator)
    }

    pub fn resolve_sthaniya(&mut self, name: Pada, dosas: &mut Vec<&'static str>) -> Option<u8> {
        for (i, sthaniya) in self.sthaniya.iter().enumerate().rev() {
            if name.mulya == sthaniya.name.mulya {
                if sthaniya.depth == -1 {
                    dosas.push("Can't read sthaniya मान in its own प्रारंभ.");
                }
                return Some(i as u8);
            }
        }
        None
    }

    pub fn resolve_gyaatmaan(&mut self, name: Pada, dosas: &mut Vec<&'static str>) -> Option<u8> {
        if let Some(enclosing) = self.enclosing.as_mut() {
            if let Some(anukram) = enclosing.resolve_sthaniya(name, dosas) {
                enclosing.sthaniya[anukram as usize].is_captured = true;
                return Some(self.add_gyaatmaan(anukram, true, dosas));
            }
            if let Some(anukram) = enclosing.resolve_gyaatmaan(name, dosas) {
                return Some(self.add_gyaatmaan(anukram, false, dosas));
            }
        }
        None
    }

    fn add_gyaatmaan(
        &mut self,
        anukram: u8,
        is_sthaniya: bool,
        dosas: &mut Vec<&'static str>,
    ) -> u8 {
        for (i, gyaatmaan) in self.sutra.gyaatmaans.iter().enumerate() {
            if gyaatmaan.anukram == anukram && gyaatmaan.is_sthaniya == is_sthaniya {
                return i as u8;
            }
        }
        let count = self.sutra.gyaatmaans.len();

        if count == Mulyankan::STHANIYA_COUNT {
            dosas.push("सूत्र में बहुत अधिक संवरण मान हैं।"); // Too many utsarga vars in sutra.
            return 0;
        }

        let gyaatmaan = SutraGyaatMaan {
            anukram,
            is_sthaniya,
        };
        self.sutra.gyaatmaans.push(gyaatmaan);
        count as u8
    }

    pub fn is_sthaniya_declared(&self, name: Pada) -> bool {
        for sthaniya in self.sthaniya.iter().rev() {
            if sthaniya.depth != -1 && sthaniya.depth < self.scope_depth {
                return false;
            }
            if sthaniya.name.mulya == name.mulya {
                return true;
            }
        }
        false
    }
}

pub struct VidhiEvaluator {
    pub enclosing: Option<Box<VidhiEvaluator>>,
    pub has_mitravidhi: bool,
}

impl VidhiEvaluator {
    pub fn new(enclosing: Option<Box<VidhiEvaluator>>) -> Box<Self> {
        Box::new(VidhiEvaluator {
            enclosing,
            has_mitravidhi: false,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Sthaniya<'p> {
    pub name: Pada<'p>,
    pub depth: i32,
    pub is_captured: bool,
}

impl<'p> Sthaniya<'p> {
    pub fn new(name: Pada<'p>, depth: i32) -> Self {
        Sthaniya {
            name,
            depth,
            is_captured: false,
        }
    }
}
