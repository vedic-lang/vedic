use std::fmt::Display;
use std::ops::Deref;
use std::ptr;
use std::u8::MAX;

use super::aadesh::Aadesh;
use super::dosa_nirupaka;
use super::dosasucana::Dosa;
use super::fetchsource::Sourcecode;
use super::logger;
use super::moolsutra::MoolSutra;
use super::mulya::Mulya;
use super::parser::Parser;
use super::smrti::Smrti;
use super::smrti_prabandhan::Gc;
use super::smrti_prabandhan::GcRef;
use super::vastuni::Avastha;
use super::vastuni::GyaatMaan;
use super::vastuni::Paddhati;
use super::vastuni::Suchi;
use super::vastuni::Utsarga;
use super::vastuni::Vakya;
use super::vastuni::Vidhi;

pub struct Aadhaar {
    pub gc: Gc,
    kosha: [Kosha; Aadhaar::ADHIKTAM_KOSHA],
    kul_kosha: usize,
    pub rashi: [Mulya; Aadhaar::ADHIKTAM_RASHI],
    rashi_murdhan: *mut Mulya,
    vaishvik: Smrti,
    gyaatmaans: Vec<GcRef<GyaatMaan>>,
    prarambhkarta_string: GcRef<Vakya>,
}

impl Aadhaar {
    const ADHIKTAM_KOSHA: usize = 64;
    const ADHIKTAM_RASHI: usize = Aadhaar::ADHIKTAM_KOSHA * (MAX as usize) + 1;

    pub fn new() -> Self {
        let mut gc = Gc::new();
        let prarambhkarta_string = gc.intern("प्रारंभ".to_owned());

        Self {
            gc,
            kosha: [Kosha {
                utsarga: GcRef::dangling(),
                ip: ptr::null(),
                slot: 0,
            }; Aadhaar::ADHIKTAM_KOSHA],
            kul_kosha: 0,
            rashi: [Mulya::Na; Aadhaar::ADHIKTAM_RASHI],
            rashi_murdhan: ptr::null_mut(),
            vaishvik: Smrti::new(),
            gyaatmaans: Vec::with_capacity(Aadhaar::ADHIKTAM_RASHI),
            prarambhkarta_string,
        }
    }

    pub fn prarambha(&mut self) {
        self.define_mool("वद", MoolSutra::vad());
        self.define_mool("पठन", MoolSutra::pathana());
        self.define_mool("समय", MoolSutra::samay());
        self.define_mool("कुल", MoolSutra::kul());
        self.define_mool("प्रकार", MoolSutra::prakaar());
        self.define_mool("निर्गम", MoolSutra::nirgam());
        self.define_mool("त्रुटि", MoolSutra::truti());
        self.define_mool("लभ्यते", MoolSutra::labhyate());
        self.define_mool("शब्द", MoolSutra::shabd());
        self.define_mool("अंक", MoolSutra::ank());
        self.rashi_murdhan = self.rashi.as_mut_ptr();
    }

    pub fn vyakhyati(&mut self, sc: Sourcecode) -> Result<(), Dosa> {
        let sutra = Parser::parse(sc, &mut self.gc)?;
        self.push(Mulya::Sutra(sutra));
        let utsarga = self.alloc(Utsarga::new(sutra));
        self.kosha[self.kul_kosha] = Kosha::new(utsarga, 0);
        self.kul_kosha += 1;
        self.run()
    }

    fn push(&mut self, v: Mulya) {
        unsafe {
            *self.rashi_murdhan = v;
            self.rashi_murdhan = self.rashi_murdhan.offset(1);
        }
    }

    fn pop(&mut self) -> Mulya {
        unsafe {
            self.rashi_murdhan = self.rashi_murdhan.offset(-1);
            *self.rashi_murdhan
        }
    }

    fn peek(&self, n: usize) -> Mulya {
        unsafe { *self.rashi_murdhan.offset(-1 - n as isize) }
    }

    fn rashi_truncate(&mut self, anukram: usize) {
        unsafe { self.rashi_murdhan = self.rashi.as_mut_ptr().add(anukram) }
    }

    pub fn rashi_len(&self) -> usize {
        unsafe { self.rashi_murdhan.offset_from(self.rashi.as_ptr()) as usize }
    }

    fn set_at(&mut self, n: usize, mulya: Mulya) {
        unsafe {
            let pos = self.rashi_murdhan.offset(-1 - (n as isize));
            *pos = mulya
        }
    }

    fn define_mool(&mut self, name: &str, mool: MoolSutra) {
        let name = self.gc.intern(name.to_owned());
        self.vaishvik.set(name, Mulya::MoolSutra(mool));
    }

    fn anusthanakale_dosa(&self, msg: &str) -> Result<(), Dosa> {
        Err(self.throw_dosa(msg))
    }

    pub fn throw_dosa(&self, msg: &str) -> Dosa {
        let current_kosha = &self.kosha[self.kul_kosha - 1];
        let dosa = format!(
            "अनुष्ठान-काले-दोषः :\n  {}\n    दोषः : {}",
            current_kosha.sthaan(),
            msg
        );
        logger::log_dosa(&dosa);
        Dosa::AnusthanaKaleDosa
    }

    fn dvimaniya_op<T>(&mut self, f: fn(f64, f64) -> T, r: fn(T) -> Mulya) -> Result<(), Dosa> {
        let operands = (self.pop(), self.pop());
        match operands {
            (Mulya::Ank(mulya_b), Mulya::Ank(mulya_a)) => {
                self.push(r(f(mulya_a, mulya_b)));
                Ok(())
            }
            _ => self.anusthanakale_dosa("अपरन्धारकाणि अवश्यं अङ्काः भवन्तु"), // Operands must be anks
        }
    }

    fn run(&mut self) -> Result<(), Dosa> {
        let mut current_kosha =
            unsafe { &mut *(&mut self.kosha[self.kul_kosha - 1] as *mut Kosha) };
        let mut current_vastu = &current_kosha.utsarga.sutra.vastu;

        loop {
            let instruction = unsafe { *current_kosha.ip };
            if let Ok(debug_mode) = std::env::var("vedic_debug") {
                if debug_mode == "full" {
                    let dis = dosa_nirupaka::Disassembler::new(
                        current_vastu,
                        Some(&self.rashi[0..self.rashi_len()]),
                    );
                    dis.op(&instruction, current_kosha.offset());
                    // dis.disassemble("vedic_debug");
                }
            }

            current_kosha.ip = unsafe { current_kosha.ip.offset(1) };
            match instruction {
                Aadesh::Cha => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Mulya::Tarka(!a.is_falsey() && !b.is_falsey()));
                }
                Aadesh::Athava => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Mulya::Tarka(!a.is_falsey() || !b.is_falsey()));
                }
                Aadesh::Yogha => {
                    let (b, a) = (self.pop(), self.pop());
                    match (&a, &b) {
                        (Mulya::Ank(a), Mulya::Ank(b)) => {
                            self.push(Mulya::Ank(a + b));
                        }

                        (Mulya::Vakya(a), Mulya::Vakya(b)) => {
                            let result = format!("{}{}", a.deref(), b.deref());
                            let result = self.intern(result);
                            let mulya = Mulya::Vakya(result);
                            self.push(mulya);
                        }
                        (Mulya::Suchi(l1), Mulya::Suchi(l2)) => {
                            let mut list_elements = l1.deref().mulyas.clone();
                            list_elements.extend(l2.deref().mulyas.clone());
                            let list = Suchi::new(list_elements);
                            let list = self.alloc(list);
                            self.push(Mulya::Suchi(list));
                        }

                        _ => {
                            self.push(a);
                            self.push(b);
                            return self.anusthanakale_dosa("अपरन्धारकाणि द्वे अंके वा द्वे वाक्ये भवन्तु");
                            // Operands must be two anks or two Vakya.
                        }
                    }
                }
                Aadesh::SuchiNirmana(size) => {
                    let mut list_elements = Vec::new();
                    for _ in 0..size {
                        list_elements.push(self.pop())
                    }
                    list_elements.reverse();
                    let list = Suchi::new(list_elements);
                    let list = self.alloc(list);
                    self.push(Mulya::Suchi(list));
                }
                Aadesh::Vidhi(achar) => {
                    let vidhi_name = current_vastu.read_vakya(achar);
                    let vidhi = Vidhi::new(vidhi_name);
                    let vidhi = self.alloc(vidhi);
                    self.push(Mulya::Vidhi(vidhi));
                }
                Aadesh::RemoveGyaatMaan => {
                    let rashi_murdhan = self.rashi_len() - 1;
                    self.close_gyaatmaans(rashi_murdhan);
                    self.pop();
                }
                Aadesh::Utsarga(achar) => {
                    let sutra = current_vastu.read_achar(achar);
                    if let Mulya::Sutra(sutra) = sutra {
                        let gyaatmaan_count = sutra.gyaatmaans.len();
                        let mut utsarga = Utsarga::new(sutra);

                        for i in 0..gyaatmaan_count {
                            let gyaatmaan = sutra.gyaatmaans[i];
                            let obj_gyaatmaan = if gyaatmaan.is_sthaniya {
                                let location = current_kosha.slot + gyaatmaan.anukram as usize;
                                self.capture_gyaatmaan(location)
                            } else {
                                current_kosha.utsarga.gyaatmaans[gyaatmaan.anukram as usize]
                            };
                            utsarga.gyaatmaans.push(obj_gyaatmaan)
                        }

                        let utsarga = self.alloc(utsarga);
                        self.push(Mulya::Utsarga(utsarga));
                    } else {
                        panic!("Utsarga instruction without sutra mulya");
                    }
                }
                Aadesh::Padaksara => {
                    let anukram = self.pop();
                    let list = self.pop();
                    match (&list, &anukram) {
                        (Mulya::Suchi(list), Mulya::Ank(anukram)) => {
                            let list = *list;
                            // add try catch for  panicked at 'anukram out of bounds: the len is 3 but the anukram is 3'
                            let mulya = list.mulyas.get(*anukram as usize);
                            match mulya {
                                Some(mulya) => self.push(*mulya),
                                None => self.push(Mulya::Anirdharita),
                            }
                        }
                        _ => {
                            return self.anusthanakale_dosa("Padaksara must be a list and an ank.");
                        }
                    }
                }
                Aadesh::SetItem => {
                    let mulya = self.pop(); // pop or peek?
                    let anukram = self.pop();
                    let list = self.peek(0);

                    match (&list, &anukram) {
                        (Mulya::Suchi(list), Mulya::Ank(anukram)) => {
                            let mut list = *list;

                            if list.length() > *anukram as usize {
                                list.set(*anukram as usize, mulya);
                            } else {
                                let dosa = format!(
                                    "anukram out of bounds: the len is {} but the anukram is {}",
                                    list.length(),
                                    anukram
                                );
                                return self.anusthanakale_dosa(&dosa);
                            }
                        }
                        _ => {
                            return self.anusthanakale_dosa("Padaksara must be a list and an ank.");
                        }
                    }
                }
                Aadesh::Ahvana(arg_count) => {
                    self.ahvana_mulya(arg_count as usize)?;
                    current_kosha =
                        unsafe { &mut *(&mut self.kosha[self.kul_kosha - 1] as *mut Kosha) };
                    current_vastu = &current_kosha.utsarga.sutra.vastu;
                }
                Aadesh::Achar(achar) => {
                    let mulya = current_vastu.read_achar(achar);
                    self.push(mulya);
                }
                Aadesh::DefineVaishvik(achar) => {
                    let vaishvik_name = current_vastu.read_vakya(achar);
                    let mulya = self.pop();
                    self.vaishvik.set(vaishvik_name, mulya);
                }
                Aadesh::Bhaga => self.dvimaniya_op(|a, b| a / b, Mulya::Ank)?,
                Aadesh::Samana => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Mulya::Tarka(a == b));
                }
                Aadesh::Asatya => self.push(Mulya::Tarka(false)),
                Aadesh::GetVaishvik(achar) => {
                    let vaishvik_name = current_vastu.read_vakya(achar);
                    match self.vaishvik.get(vaishvik_name) {
                        Some(mulya) => self.push(mulya),
                        None => {
                            let msg = format!("अपरिभाषितं मान '{}'.", vaishvik_name.deref()); // Undefined मान
                            return self.anusthanakale_dosa(&msg);
                        }
                    }
                }
                Aadesh::GetSthaniya(slot) => {
                    let i = slot as usize + current_kosha.slot;
                    let mulya = self.rashi[i];
                    self.push(mulya);
                }
                Aadesh::GetGuna(achar) => {
                    if let Mulya::Avastha(avastha) = self.peek(0) {
                        let vidhi = avastha.vidhi;
                        let guna_name = current_vastu.read_vakya(achar);
                        let mulya = avastha.fields.get(guna_name);
                        match mulya {
                            Some(mulya) => {
                                self.pop();
                                self.push(mulya);
                            }
                            None => {
                                self.bind_paddhati(vidhi, guna_name)?;
                            }
                        }
                    } else {
                        return self.anusthanakale_dosa("केवल अवस्थाः एव गुणाः अस्ति");
                        // Only avasthas have properties.
                    }
                }
                Aadesh::GetMitra(achar) => {
                    let paddhati_name = current_vastu.read_vakya(achar);
                    if let Mulya::Vidhi(mitravidhi) = self.pop() {
                        self.bind_paddhati(mitravidhi, paddhati_name)?;
                    } else {
                        panic!("मित्रः न विधिमश्नाति"); // mitra found no vidhi
                    }
                }
                Aadesh::GetGyaatMaan(slot) => {
                    let mulya = {
                        let gyaatmaan = current_kosha.utsarga.gyaatmaans[slot as usize];
                        if let Some(mulya) = gyaatmaan.closed {
                            mulya
                        } else {
                            self.rashi[gyaatmaan.location]
                        }
                    };
                    self.push(mulya);
                }
                Aadesh::Mahattara => self.dvimaniya_op(|a, b| a > b, Mulya::Tarka)?,
                Aadesh::Anuharana => {
                    let pair = (self.peek(0), self.peek(1));
                    if let (Mulya::Vidhi(mut subvidhi), Mulya::Vidhi(mitravidhi)) = pair {
                        subvidhi.paddhatis = Smrti::new();
                        subvidhi.paddhatis.add_all(&mitravidhi.paddhatis);
                        self.pop();
                    } else {
                        return self.anusthanakale_dosa("मित्र-विधि: एक विधि भविष्यति।");
                        // mitravidhi must be a vidhi
                    }
                }
                Aadesh::Avacate((achar, arg_count)) => {
                    let name = current_vastu.read_vakya(achar);
                    self.invoke(name, arg_count as usize)?;
                    current_kosha =
                        unsafe { &mut *(&mut self.kosha[self.kul_kosha - 1] as *mut Kosha) };
                    current_vastu = &current_kosha.utsarga.sutra.vastu;
                }
                Aadesh::Viraam => {
                    let mut offset = 0;
                    loop {
                        let ins = unsafe { current_kosha.ip.offset(offset) };

                        match unsafe { *ins } {
                            Aadesh::Pratigam(_) => {
                                current_kosha.ip = unsafe { current_kosha.ip.offset(offset + 2) };

                                break;
                            }
                            Aadesh::Phala => {
                                return self.anusthanakale_dosa("विराम केवलं एके चक्रे/पर्यन्ते उपयुज्यते।");
                                // 'break' can only be used inside a for/while loop.
                            }
                            _ => offset += 1,
                        }
                    }
                }
                Aadesh::Agrim => {
                    let mut offset = 0;
                    loop {
                        let ins = unsafe { current_kosha.ip.offset(offset) };

                        match unsafe { *ins } {
                            Aadesh::Pratigam(_) => {
                                current_kosha.ip = unsafe { current_kosha.ip.offset(offset) };
                                break;
                            }
                            Aadesh::Phala => {
                                return self.anusthanakale_dosa("अग्रिम् केवलं एके चक्रे/पर्यन्ते उपयुज्यते");
                                // 'continue' can only be used inside a for/while loop.
                            }
                            _ => offset += 1,
                        }
                    }
                }
                Aadesh::Agresit(offset) => {
                    current_kosha.ip = unsafe { current_kosha.ip.offset(offset as isize) };
                }
                Aadesh::AgresitYadiAsatya(offset) => {
                    if self.peek(0).is_falsey() {
                        current_kosha.ip = unsafe { current_kosha.ip.offset(offset as isize) };
                    }
                }
                Aadesh::Laghutara => self.dvimaniya_op(|a, b| a < b, Mulya::Tarka)?,
                Aadesh::Pratigam(offset) => {
                    current_kosha.ip = unsafe { current_kosha.ip.offset(-1 - (offset as isize)) };
                }

                Aadesh::Sheshaphal => self.dvimaniya_op(|a, b| a % b, Mulya::Ank)?,
                Aadesh::Paddhati(achar) => {
                    let paddhati_name = current_vastu.read_vakya(achar);
                    self.define_paddhati(paddhati_name);
                }
                Aadesh::Gunayati => self.dvimaniya_op(|a, b| a * b, Mulya::Ank)?,
                Aadesh::Rnaatmak => {
                    if let Mulya::Ank(mulya) = self.peek(0) {
                        self.pop();
                        self.push(Mulya::Ank(-mulya));
                    } else {
                        return self.anusthanakale_dosa("अङ्कः अभिलक्षणं भवन्निति आवश्यकम्");
                        // Operand must be a ank
                    }
                }
                Aadesh::Na => self.push(Mulya::Na),
                Aadesh::Not => {
                    let mulya = self.pop();
                    self.push(Mulya::Tarka(mulya.is_falsey()));
                }
                Aadesh::Pop => {
                    self.pop();
                }
                Aadesh::Phala => {
                    self.kul_kosha -= 1;
                    let phala_mulya = self.pop();
                    self.close_gyaatmaans(current_kosha.slot);

                    if self.kul_kosha == 0 {
                        return Ok(());
                    } else {
                        self.rashi_truncate(current_kosha.slot);
                        self.push(phala_mulya);

                        current_kosha =
                            unsafe { &mut *(&mut self.kosha[self.kul_kosha - 1] as *mut Kosha) };
                        current_vastu = &current_kosha.utsarga.sutra.vastu;
                    }
                }
                Aadesh::SetVaishvik(achar) => {
                    let vaishvik_name = current_vastu.read_vakya(achar);
                    let mulya = self.peek(0);
                    if self.vaishvik.set(vaishvik_name, mulya) {
                        self.vaishvik.delete(vaishvik_name);
                        let msg = format!("अपरिभाषितं मान '{}'.", vaishvik_name.deref()); // Undefined मान
                        return self.anusthanakale_dosa(&msg);
                    }
                }
                Aadesh::SetSthaniya(slot) => {
                    let i = slot as usize + (current_kosha).slot;
                    let mulya = self.peek(0);
                    self.rashi[i] = mulya;
                }
                Aadesh::SetGuna(achar) => {
                    if let Mulya::Avastha(mut avastha) = self.peek(1) {
                        let guna_name = current_vastu.read_vakya(achar);
                        let mulya = self.pop();
                        avastha.fields.set(guna_name, mulya);
                        self.pop();
                        self.push(mulya);
                    } else {
                        return self.anusthanakale_dosa("अवस्थाः एव क्षेत्राणि भवन्ति");
                        // Only avasthas have fields
                    }
                }
                Aadesh::SetGyaatMaan(slot) => {
                    let mut gyaatmaan = current_kosha.utsarga.gyaatmaans[slot as usize];
                    let mulya = self.peek(0);
                    if gyaatmaan.closed.is_none() {
                        self.rashi[gyaatmaan.location] = mulya;
                    } else {
                        gyaatmaan.closed = Some(mulya);
                    }
                }
                Aadesh::Vyavakalana => self.dvimaniya_op(|a, b| a - b, Mulya::Ank)?,
                Aadesh::MitraAvacate((achar, arg_count)) => {
                    let paddhati_name = current_vastu.read_vakya(achar);
                    if let Mulya::Vidhi(vidhi) = self.pop() {
                        self.invoke_from_vidhi(vidhi, paddhati_name, arg_count as usize)?;
                        current_kosha =
                            unsafe { &mut *(&mut self.kosha[self.kul_kosha - 1] as *mut Kosha) };
                        current_vastu = &current_kosha.utsarga.sutra.vastu;
                    } else {
                        panic!("मित्रं नविधिना अवोकय"); // mitra invoke with no vidhi
                    }
                }
                Aadesh::Satya => self.push(Mulya::Tarka(true)),
            };
        }
    }

    fn ahvana_mulya(&mut self, arg_count: usize) -> Result<(), Dosa> {
        match self.peek(arg_count) {
            Mulya::Paddhati(bound) => {
                let paddhati = bound.paddhati;
                let adatr = bound.adatr;
                self.set_at(arg_count, adatr);
                self.ahvana(paddhati, arg_count)
            }
            Mulya::Vidhi(vidhi) => {
                let avastha = Avastha::new(vidhi);
                let avastha = self.alloc(avastha);
                self.set_at(arg_count, Mulya::Avastha(avastha));
                if let Some(prarambhkarta) = vidhi.paddhatis.get(self.prarambhkarta_string) {
                    if let Mulya::Utsarga(prarambhkarta) = prarambhkarta {
                        return self.ahvana(prarambhkarta, arg_count);
                    }
                    return self.anusthanakale_dosa("प्रारंभकर्ता न उत्सर्गः"); // Prarambhkarta is not utsarga
                } else if arg_count != 0 {
                    let msg = format!("शून्यं निरूपकं अपेक्षिता प्राप्तं  {arg_count}"); // Expected 0 arguments but got {arg_count}
                    return self.anusthanakale_dosa(&msg);
                }
                Ok(())
            }
            Mulya::Utsarga(utsarga) => self.ahvana(utsarga, arg_count),
            Mulya::MoolSutra(mool) => {
                let from = self.rashi_len() - arg_count;
                let result = mool.0(self, from);
                self.rashi_truncate(from - 1);
                match result {
                    Ok(mulya) => {
                        self.push(mulya);
                        Ok(())
                    }
                    Err(dosa) => Err(dosa),
                }
            }
            _ => self.anusthanakale_dosa("केवलं सूत्रं च विधिं च उक्त्वा वदन्तु शक्त्यः"), /* Can only call सूत्र and विधि */
        }
    }

    fn ahvana(&mut self, utsarga: GcRef<Utsarga>, arg_count: usize) -> Result<(), Dosa> {
        let sutra = utsarga.sutra;
        if arg_count != sutra.arity {
            let msg = format!(
                "अपेक्षितानि {} विभक्तीषु प्रतीयमानानि {} विभक्तीषु सन्ति",
                sutra.arity, arg_count
            ); // expected x got y
            self.anusthanakale_dosa(&msg)
        } else if self.kul_kosha == Aadhaar::ADHIKTAM_KOSHA {
            self.anusthanakale_dosa("राशि अतिर्वर्तनम्") // Rashi overflow
        } else {
            let kosha = Kosha::new(utsarga, self.rashi_len() - arg_count - 1);
            self.kosha[self.kul_kosha] = kosha;
            self.kul_kosha += 1;
            Ok(())
        }
    }

    fn invoke(&mut self, name: GcRef<Vakya>, arg_count: usize) -> Result<(), Dosa> {
        let adatr = self.peek(arg_count);

        if let Mulya::Avastha(avastha) = adatr {
            if let Some(field) = avastha.fields.get(name) {
                self.set_at(arg_count, field);
                self.ahvana_mulya(arg_count)
            } else {
                let vidhi = avastha.vidhi;
                self.invoke_from_vidhi(vidhi, name, arg_count)
            }
        } else if let Mulya::Suchi(mut list) = adatr {
            match name.deref().s.as_str() {
                "आगम" => {
                    for _i in 0..arg_count {
                        let val = self.pop();
                        list.mulyas.push(val);
                    }
                }
                "निर्गम" => {
                    if arg_count == 0 {
                        self.pop();
                        if let Some(val) = list.mulyas.pop() {
                            self.push(val);
                        } else {
                            self.push(Mulya::Anirdharita);
                        }
                    }
                }
                paddhati_name => {
                    return self.anusthanakale_dosa(
                        format!("{} सूचिस्य पद्धतिरन्यत् न भवति", paddhati_name).as_str(), //is not a paddhati of list
                    );
                }
            }
            Ok(())
        } else {
            self.anusthanakale_dosa("एवं मात्रावस्थासु पद्धतय: अस्ति") // Only avasthas have paddhatis.
        }
    }

    fn invoke_from_vidhi(
        &mut self,
        vidhi: GcRef<Vidhi>,
        name: GcRef<Vakya>,
        arg_count: usize,
    ) -> Result<(), Dosa> {
        if let Some(paddhati) = vidhi.paddhatis.get(name) {
            if let Mulya::Utsarga(utsarga) = paddhati {
                self.ahvana(utsarga, arg_count)
            } else {
                panic!("उत्सर्ग रहिता प्राप्ता पद्धतिः!") // Got paddhati that is not utsarga
            }
        } else {
            let msg = format!("अपरिभाषितं गुण : '{}'", name.deref());
            self.anusthanakale_dosa(&msg)
        }
    }

    fn bind_paddhati(&mut self, vidhi: GcRef<Vidhi>, name: GcRef<Vakya>) -> Result<(), Dosa> {
        if let Some(paddhati) = vidhi.paddhatis.get(name) {
            let adatr = self.peek(0);
            match paddhati {
                Mulya::Utsarga(paddhati) => {
                    let bound = Paddhati::new(adatr, paddhati);
                    let bound = self.alloc(bound);
                    self.pop();
                    self.push(Mulya::Paddhati(bound));
                    Ok(())
                }
                _ => panic!("असन्दिग्धा अवस्था: पद्धति न उत्सर्ग अस्ति"), /* Inconsistent state. Paddhati is not utsarga */
            }
        } else {
            let msg = format!("अपरिभाषितं गुण '{}'.", name.deref()); // Undefined गुण
            self.anusthanakale_dosa(&msg)
        }
    }

    fn capture_gyaatmaan(&mut self, location: usize) -> GcRef<GyaatMaan> {
        for &gyaatmaan in &self.gyaatmaans {
            if gyaatmaan.location == location {
                return gyaatmaan;
            }
        }
        let gyaatmaan = GyaatMaan::new(location);
        let gyaatmaan = self.alloc(gyaatmaan);
        self.gyaatmaans.push(gyaatmaan);
        gyaatmaan
    }

    fn close_gyaatmaans(&mut self, last: usize) {
        let mut i = 0;
        while i != self.gyaatmaans.len() {
            let mut gyaatmaan = self.gyaatmaans[i];
            if gyaatmaan.location >= last {
                // PERF: Remove is expensive
                self.gyaatmaans.remove(i);
                let location = gyaatmaan.location;
                gyaatmaan.closed = Some(self.rashi[location]);
            } else {
                i += 1;
            }
        }
    }

    fn define_paddhati(&mut self, name: GcRef<Vakya>) {
        let paddhati = self.peek(0);
        if let Mulya::Vidhi(mut vidhi) = self.peek(1) {
            vidhi.paddhatis.set(name, paddhati);
            self.pop();
        } else {
            panic!("नविधि रहितस्य पद्धति परिभाषितुं प्रयतन्तः अयुक्तिः अवस्था:"); // Invalid state: trying to define a paddhati of non vidhi
        }
    }

    fn alloc<T: Display + 'static>(&mut self, vastu: T) -> GcRef<T> {
        self.mark_and_sweep();
        self.gc.alloc(vastu)
    }

    fn intern(&mut self, name: String) -> GcRef<Vakya> {
        self.mark_and_sweep();
        self.gc.intern(name)
    }

    fn mark_and_sweep(&mut self) {
        if self.gc.should_gc() {
            self.mark_roots();
            self.gc.collect_garbage();
        }
    }

    fn mark_roots(&mut self) {
        for &mulya in &self.rashi[0..self.rashi_len()] {
            self.gc.mark_mulya(mulya);
        }

        for kosha in &self.kosha[..self.kul_kosha] {
            self.gc.mark_vastu(kosha.utsarga)
        }

        for &gyaatmaan in &self.gyaatmaans {
            self.gc.mark_vastu(gyaatmaan);
        }

        self.gc.mark_smrti(&self.vaishvik);
        self.gc.mark_vastu(self.prarambhkarta_string);
    }
}

#[derive(Clone, Copy)]
struct Kosha {
    utsarga: GcRef<Utsarga>,
    ip: *const Aadesh,
    slot: usize,
}

impl Kosha {
    fn new(utsarga: GcRef<Utsarga>, slot: usize) -> Self {
        Kosha {
            utsarga,
            ip: utsarga.sutra.vastu.code.as_ptr(),
            slot,
        }
    }

    fn offset(&self) -> usize {
        unsafe {
            let vastu = &self.utsarga.sutra.vastu;
            let pos = self.ip.offset_from(vastu.code.as_ptr());
            pos as usize
        }
    }

    fn sthaan(&self) -> &str {
        self.utsarga.sutra.vastu.sthaan[self.offset() - 1].as_str()
    }
}
