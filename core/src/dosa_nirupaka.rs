use super::aadesh::Aadesh;
use super::mulya::Mulya;
use super::vastuni::Vastu;

pub struct Disassembler<'vm> {
    vastu: &'vm Vastu,
    rashi: Option<&'vm [Mulya]>,
}

impl<'vm> Disassembler<'vm> {
    fn const_aadesh(&self, op: &str, achar_anukram: u8) {
        let mulya = self.vastu.achar[achar_anukram as usize];
        println!("{:<16} {:4} ({})", op, achar_anukram, mulya);
    }

    fn const_list_aadesh(&self, op: &str, achar_anukram: u16) {
        let mulya = self.vastu.achar[achar_anukram as usize];
        println!("{:<16} {:4} ({})", op, achar_anukram, mulya);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== BEGIN {} ==", name);
        for (offset, op) in self.vastu.code.iter().enumerate() {
            self.op(op, offset);
        }
        println!("== END {} ==", name);
        println!();
    }

    fn invoke_aadesh(&self, op: &str, achar_anukram: u8, args: u8) {
        let mulya = self.vastu.achar[achar_anukram as usize];
        println!("{:<16} {:4} ({}) {}", op, achar_anukram, mulya, args);
    }

    fn jump_aadesh(&self, op: &str, offset: u16) {
        println!("{:<16} {:4}", op, offset);
    }

    pub fn new(vastu: &'vm Vastu, rashi: Option<&'vm [Mulya]>) -> Self {
        Disassembler { vastu, rashi }
    }

    pub fn op(&self, op: &Aadesh, offset: usize) {
        self.rashi();
        print!("{:04} ", offset);
        let line = &self.vastu.sthaan[offset];
        if offset > 0 && line == &self.vastu.sthaan[offset - 1] {
            print!("   | ");
        } else {
            print!("{:>4} ", line);
        }
        match op {
            Aadesh::Vidhi(c) => self.const_aadesh("Vidhi", *c),
            Aadesh::Utsarga(c) => self.const_aadesh("Utsarga", *c),
            Aadesh::Achar(c) => self.const_aadesh("Achar", *c),
            Aadesh::DefineVaishvik(c) => self.const_aadesh("DefineVaishvik", *c),
            Aadesh::GetVaishvik(c) => self.const_aadesh("GetVaishvik", *c),
            Aadesh::GetSthaniya(s) => self.slot_aadesh("GetSthaniya", *s),
            Aadesh::GetGuna(c) => self.const_aadesh("GetGuna", *c),
            Aadesh::GetMitra(c) => self.const_aadesh("GetMitra", *c),
            Aadesh::GetGyaatMaan(s) => self.slot_aadesh("GetGyaatMaan", *s),
            Aadesh::Avacate((c, args)) => self.invoke_aadesh("Avacate", *c, *args),
            Aadesh::Agresit(offset) => self.jump_aadesh("Agresit", *offset),
            Aadesh::AgresitYadiAsatya(offset) => self.jump_aadesh("AgresitYadiAsatya", *offset),
            Aadesh::Pratigam(offset) => self.jump_aadesh("Pratigam", *offset),
            Aadesh::Paddhati(c) => self.const_aadesh("Paddhati", *c),
            Aadesh::SetVaishvik(c) => self.const_aadesh("SetVaishvik", *c),
            Aadesh::SetSthaniya(s) => self.slot_aadesh("SetSthaniya", *s),
            Aadesh::SetGuna(c) => self.const_aadesh("SetGuna", *c),
            Aadesh::SetGyaatMaan(s) => self.slot_aadesh("SetGyaatMaan", *s),
            Aadesh::MitraAvacate((c, args)) => self.invoke_aadesh("MitraAvacate", *c, *args),
            Aadesh::SuchiNirmana(id) => self.const_list_aadesh("SuchiNirmana", *id),
            Aadesh::Ahvana(id) => self.const_aadesh("Ahvana", *id),
            Aadesh::Cha => println!("Cha"),
            Aadesh::Yogha => println!("Yoga"),
            Aadesh::Athava => println!("Athava"),
            Aadesh::Agrim => println!("Agrim"),
            Aadesh::RemoveGyaatMaan => println!("RemoveGyaatMaan"),
            Aadesh::Bhaga => println!("Bhaga"),
            Aadesh::Samana => println!("Samana"),
            Aadesh::Asatya => println!("Asatya"),
            Aadesh::Mahattara => println!("Mahattara"),
            Aadesh::Anuharana => println!("Anuharana"),
            Aadesh::Laghutara => println!("Laghutara"),
            Aadesh::Sheshaphal => println!("Sheshaphal"),
            Aadesh::Gunayati => println!("Gunayati"),
            Aadesh::Rnaatmak => println!("Rnaatmak"),
            Aadesh::Na => println!("Na"),
            Aadesh::Not => println!("Not"),
            Aadesh::Pop => println!("Pop"),
            Aadesh::Phala => println!("Phala"),
            Aadesh::Padaksara => println!("Padaksara"),
            Aadesh::Vyavakalana => println!("Vyavakalana"),
            Aadesh::Satya => println!("Satya"),
            Aadesh::Viraam => println!("Viraam"),
            Aadesh::SetItem => println!("SetItem"),
        }
    }

    fn slot_aadesh(&self, op: &str, slot: u8) {
        println!("{:<16} {:4}", op, slot);
    }

    fn rashi(&self) {
        if let Some(rashi) = self.rashi {
            println!(" S: ");
            for &mulya in rashi.iter() {
                println!("\t[{}]", mulya);
            }
            println!();
        }
    }
}
