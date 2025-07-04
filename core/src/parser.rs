use std::convert::TryFrom;
use std::mem;
use std::path::Path;

use super::aadesh::Aadesh;
use super::anupurva::AnupUrva;
use super::dosasucana::Dosa;
use super::fetchsource::Sourcecode;
use super::lexer::Lexer;
use super::logger;
use super::mulya::Mulya;
use super::mulyankan::Mulyankan;
use super::mulyankan::Sthaniya;
use super::mulyankan::VidhiEvaluator;
use super::pada::Pada;
use super::pada::PadaPrakara;
use super::rules::ParseRule;
use super::smrti_prabandhan::Gc;
use super::smrti_prabandhan::GcRef;
use super::vastuni::Sutra;
use super::vastuni::SutraType;

pub struct Parser<'p> {
    lexer: Lexer<'p>,
    evaluator: Box<Mulyankan<'p>>,
    vidhi_evaluator: Option<Box<VidhiEvaluator>>,
    gc: &'p mut Gc,
    current: Pada<'p>,
    previous: Pada<'p>,
    had_dosa: bool,
    panic_mode: bool,
    resolver_dosas: Vec<&'static str>,
}

impl<'p> Parser<'p> {
    pub fn parse(source: Sourcecode, gc: &mut Gc) -> Result<GcRef<Sutra>, Dosa> {
        let sutra_name = gc.intern(source.path.to_owned());
        let parser = Parser {
            lexer: Lexer::new(&source),
            evaluator: Mulyankan::new(sutra_name, SutraType::MoolLekha),
            vidhi_evaluator: None,
            gc,
            current: Pada::default(""),
            previous: Pada::default(""),
            had_dosa: false,
            panic_mode: false,
            resolver_dosas: Vec::new(),
        };
        parser.start()
    }

    fn start(mut self) -> Result<GcRef<Sutra>, Dosa> {
        self.read_next();

        while !self.matches(PadaPrakara::Eof) {
            self.declaration();
        }

        self.emit_phala();
        if self.had_dosa {
            Err(Dosa::SagkalanaKaleDosa)
        } else {
            Ok(self.gc.alloc(self.evaluator.sutra))
        }
    }

    fn expression(&mut self) {
        self.parse_anupurva(AnupUrva::Niyojana);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(PadaPrakara::ArdhaViram, "व्याख्या पश्चात् ';' अपेक्षितः"); // Expect ';' after expression.
        self.emit(Aadesh::Pop);
    }

    fn declaration(&mut self) {
        if self.matches(PadaPrakara::Vidhi) {
            self.vidhi_declaration();
        } else if self.matches(PadaPrakara::Sutra) {
            self.sutra_declaration();
        } else if self.matches(PadaPrakara::Maan) {
            self.maan_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn vidhi_declaration(&mut self) {
        self.consume(PadaPrakara::Identifier, "विधि नामानि अपेक्षितः"); // Expect vidhi name.
        let vidhi_name = self.previous;
        let name_achar = self.identifier_achar(vidhi_name);
        self.declare_maan();
        self.emit(Aadesh::Vidhi(name_achar));
        self.define_maan(name_achar);

        let old_vidhi_evaluator = self.vidhi_evaluator.take();
        let new_vidhi_evaluator = VidhiEvaluator::new(old_vidhi_evaluator);
        self.vidhi_evaluator.replace(new_vidhi_evaluator);

        if self.matches(PadaPrakara::Sandhi) {
            self.consume(
                PadaPrakara::Identifier,
                "संधि पश्चात् मित्र-विधिनाम् अपेक्षितः", // Expect मित्र-विधि name after Sandhi.
            );
            self.maan(false);
            if vidhi_name.mulya == self.previous.mulya {
                self.dosa("विधिः स्वयम् इत्थंभूतम् नानुवाद्यते"); // A vidhi can't inherit from itself
            }
            self.begin_scope();
            self.add_sthaniya(Pada::default("मित्र"));
            self.define_maan(0);
            self.named_maan(vidhi_name, false);
            self.emit(Aadesh::Anuharana);
            self.vidhi_evaluator.as_mut().unwrap().has_mitravidhi = true;
        }

        self.named_maan(vidhi_name, false);
        self.consume(PadaPrakara::VamMadhyamKoshthak, "विधि पूर्वं '{' अपेक्षितः"); // Expect '{' before vidhi body.
        while !self.check(PadaPrakara::DaksinaMadhyamKoshthak) && !self.check(PadaPrakara::Eof) {
            self.paddhati();
        }
        self.consume(
            PadaPrakara::DaksinaMadhyamKoshthak,
            "विधि पश्चात् '}' अपेक्षितः",
        ); // Expect '}' after vidhi body.
        self.emit(Aadesh::Pop);
        if self.vidhi_evaluator.as_ref().unwrap().has_mitravidhi {
            self.end_scope();
        }

        match self.vidhi_evaluator.take() {
            Some(c) => self.vidhi_evaluator = c.enclosing,
            None => self.vidhi_evaluator = None,
        }
    }

    fn sutra_declaration(&mut self) {
        let vaishvik = self.parse_maan("सूत्रनाम् अपेक्षितः"); // Expect sutra name.
        self.mark_prarambha();
        self.sutra(SutraType::Sutra);
        self.define_maan(vaishvik);
    }

    fn push_evaluator(&mut self, kind: SutraType) {
        let sutra_name = self.gc.intern(self.previous.mulya.to_owned());
        let new_evaluator = Mulyankan::new(sutra_name, kind);
        let old_evaluator = mem::replace(&mut self.evaluator, new_evaluator);
        self.evaluator.enclosing = Some(old_evaluator);
    }

    fn pop_evaluator(&mut self) -> Sutra {
        self.emit_phala();
        match self.evaluator.enclosing.take() {
            Some(enclosing) => {
                let evaluator = mem::replace(&mut self.evaluator, enclosing);
                evaluator.sutra
            }
            None => panic!("मूल्यांकाङ्कर्तृकं अन्वेषमाणो न पश्यामि"), // Didn't find an enclosing evaluator
        }
    }

    fn sutra(&mut self, kind: SutraType) {
        self.push_evaluator(kind);
        self.begin_scope();
        self.consume(
            PadaPrakara::VamLaghuKoshthak,
            "सूत्रनामन्तरे '(' अपेक्षितः", // Expect '(' after sutra name
        );
        if !self.check(PadaPrakara::DaksinaLaghuKoshthak) {
            loop {
                self.evaluator.sutra.arity += 1;
                if self.evaluator.sutra.arity > 255 {
                    self.dosa_at_current("परिमिति २५५ अधिकं अपि न भवेत्"); // Can't have more than 255 parameters.
                }
                let param = self.parse_maan("परिमितिनाम् अपेक्षितः"); // Expect parameter name
                self.define_maan(param);
                if !self.matches(PadaPrakara::AlpViram) {
                    break;
                }
            }
        }
        self.consume(
            PadaPrakara::DaksinaLaghuKoshthak,
            "परिमितिनाम् पश्चात् ')' अपेक्षितः", //  Expect ')' after parameters
        );
        self.consume(
            PadaPrakara::VamMadhyamKoshthak,
            "सूत्रम् पूर्वं '{' अपेक्षितः", // Expect '{' before sutra body
        );
        self.block();
        let sutra = self.pop_evaluator();
        let fn_id = self.gc.alloc(sutra);

        let anukram = self.make_achar(Mulya::Sutra(fn_id));
        self.emit(Aadesh::Utsarga(anukram));
    }

    fn paddhati(&mut self) {
        self.consume(PadaPrakara::Identifier, "पद्धतिनाम् अपेक्षितः"); // Expect paddhati name
        let achar = self.identifier_achar(self.previous);
        let vastuni = if self.previous.mulya == "प्रारंभ" {
            SutraType::Mool
        } else {
            SutraType::Paddhati
        };
        self.sutra(vastuni);
        self.emit(Aadesh::Paddhati(achar));
    }

    fn maan_declaration(&mut self) {
        let anukram = self.parse_maan("मान नाम् अपेक्षितः"); // Expect maan name
        if self.matches(PadaPrakara::Barabar) {
            self.expression();
        } else {
            self.emit(Aadesh::Na);
        }
        self.consume(PadaPrakara::ArdhaViram, "मान घोषणायाः पश्चात् ';' अपेक्षितः"); // Expect ';' after मान declaration
        self.define_maan(anukram);
    }

    fn define_maan(&mut self, anukram: u8) {
        if self.evaluator.scope_depth > 0 {
            self.mark_prarambha();
            return;
        }
        self.emit(Aadesh::DefineVaishvik(anukram));
    }

    fn mark_prarambha(&mut self) {
        if self.evaluator.scope_depth == 0 {
            return;
        }
        let last_sthaniya = self.evaluator.sthaniya.last_mut().unwrap();
        last_sthaniya.depth = self.evaluator.scope_depth;
    }

    fn statement(&mut self) {
        if self.matches(PadaPrakara::Yadi) {
            self.yadi_statement();
        } else if self.matches(PadaPrakara::Phala) {
            self.phala_statement();
        } else if self.matches(PadaPrakara::Paryant) {
            self.paryant_statement();
        } else if self.matches(PadaPrakara::Chakra) {
            self.chakra_statement();
        } else if self.matches(PadaPrakara::Viram) {
            self.viram_statement();
        } else if self.matches(PadaPrakara::Agrim) {
            self.continue_statement();
        } else if self.matches(PadaPrakara::VamMadhyamKoshthak) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else if self.matches(PadaPrakara::Import) {
            self.import_statement();
        } else {
            self.expression_statement();
        }
    }

    fn import_statement(&mut self) {
        self.consume(PadaPrakara::String, "अवहन पश्चात् नाम् अपेक्षितः"); // Expect string after import
        let mulya = self.previous.mulya;
        let relative_filepath = &mulya[1..(mulya.len() - 1)];
        let import_path = Path::new(&self.previous.path)
            .parent()
            .unwrap()
            .join(relative_filepath);

        match Sourcecode::new(import_path.to_str().unwrap()) {
            Ok(import_sc) => match Parser::parse(import_sc, self.gc) {
                Ok(import_sutra_id) => {
                    let anukram = self.make_achar(Mulya::Sutra(import_sutra_id));
                    self.emit(Aadesh::Utsarga(anukram));
                    self.emit(Aadesh::Ahvana(0));
                    self.emit(Aadesh::Pop);
                }
                Err(_dosa) => {
                    let msg = format!("सञ्चिकाः आवाहनं कर्तुं असमर्थः : {relative_filepath}"); // Unable to import file
                    self.dosa(&msg);
                }
            },
            Err(dosa) => {
                let msg = format!("सञ्चिकाः आवाहनं कर्तुं असमर्थः : {dosa}"); // Unable to import file
                self.dosa(&msg);
            }
        }
        self.consume(PadaPrakara::ArdhaViram, "अवहन पश्चात् ';' अपेक्षितः"); // Expect ';' after import
    }

    fn phala_statement(&mut self) {
        if let SutraType::MoolLekha = self.evaluator.sutra.kind {
            self.dosa("मूर्धन्स्थला फलः प्रतिगच्छामि न शक्नोमि"); // Can't phala from topmurdhan-level code //todo
        }
        if self.matches(PadaPrakara::ArdhaViram) {
            self.emit_phala();
        } else {
            if let SutraType::Mool = self.evaluator.sutra.kind {
                self.dosa("प्रारंभकर्ता फलः प्रतिगच्छामि न शक्नोमि"); // Can't return a mulya from an prarambhkarta.
            }
            self.expression();
            self.consume(PadaPrakara::ArdhaViram, "फल मूल्य पश्चात् ';' अपेक्षितः"); // Expect ';' after return mulya.
            self.emit(Aadesh::Phala);
        }
    }

    fn yadi_statement(&mut self) {
        self.consume(PadaPrakara::VamLaghuKoshthak, "'यदि' पश्चात् '(' अपेक्षितः"); // Expect '(' after 'yadi'.
        self.expression();
        self.consume(
            PadaPrakara::DaksinaLaghuKoshthak,
            "नियम पश्चात् ')' अपेक्षितः", // Expect ')' after condition.
        );
        let then_jump = self.emit(Aadesh::AgresitYadiAsatya(0xFFFF));
        self.emit(Aadesh::Pop);
        self.statement();
        let atha_jump = self.emit(Aadesh::Agresit(0xFFFF));
        self.patch_jump(then_jump);
        self.emit(Aadesh::Pop);
        if self.matches(PadaPrakara::Atha) {
            self.statement();
        }
        self.patch_jump(atha_jump);
    }

    fn paryant_statement(&mut self) {
        let loop_start = self.op_pos();
        self.consume(PadaPrakara::VamLaghuKoshthak, "पर्यन्त पश्चात् '(' अपेक्षितः"); // Expect '(' after 'paryant'
        self.expression();
        self.consume(
            PadaPrakara::DaksinaLaghuKoshthak,
            "नियम पश्चात् ')' अपेक्षितः", // Expect ')' after condition."
        );
        let exit_jump = self.emit(Aadesh::AgresitYadiAsatya(0xFFFF));
        self.emit(Aadesh::Pop);
        self.statement();
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit(Aadesh::Pop);
    }

    fn viram_statement(&mut self) {
        self.consume(PadaPrakara::ArdhaViram, "'विराम' पश्चात् ')' अपेक्षितः"); // Expect ';' after 'विराम'
        self.emit(Aadesh::Viraam);
    }

    fn continue_statement(&mut self) {
        self.consume(PadaPrakara::ArdhaViram, "'अग्रिम' पश्चात् ';' अपेक्षितः"); // Expect ';' after 'अग्रिम'
        self.emit(Aadesh::Agrim);
    }

    fn chakra_statement(&mut self) {
        self.begin_scope();
        self.consume(PadaPrakara::VamLaghuKoshthak, "'चक्र' पश्चात् '(' अपेक्षितः"); // Expect '(' after 'चक्र'.

        // Prarambhkarta
        if self.matches(PadaPrakara::ArdhaViram) {
            // no prarambhkarta
        } else if self.matches(PadaPrakara::Maan) {
            self.maan_declaration();
        } else {
            self.expression_statement();
        }
        let mut loop_start = self.op_pos();

        // Condition
        let mut exit_jump = Option::None;
        if !self.matches(PadaPrakara::ArdhaViram) {
            self.expression();
            self.consume(PadaPrakara::ArdhaViram, "चक्र नियम पश्चात् '(' अपेक्षितः"); // Expect ';' after loop condition
            let jump = self.emit(Aadesh::AgresitYadiAsatya(0xFFFF));
            exit_jump = Option::from(jump);
            self.emit(Aadesh::Pop);
        }

        // Increment
        if !self.matches(PadaPrakara::DaksinaLaghuKoshthak) {
            let body_jump = self.emit(Aadesh::Agresit(0xFFFF));
            let increment_start = self.op_pos();
            self.expression();
            self.emit(Aadesh::Pop);
            self.consume(
                PadaPrakara::DaksinaLaghuKoshthak,
                "चक्र पश्चात् ')' अपेक्षितः", // Expect ')' after चक्र clauses
            );
            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }
        self.statement();
        self.emit_loop(loop_start);
        if let Option::Some(exit_jump) = exit_jump {
            self.patch_jump(exit_jump);
            self.emit(Aadesh::Pop);
        }
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.evaluator.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.evaluator.scope_depth -= 1;
        for i in (0..self.evaluator.sthaniya.len()).rev() {
            if self.evaluator.sthaniya[i].depth > self.evaluator.scope_depth {
                if self.evaluator.sthaniya[i].is_captured {
                    self.emit(Aadesh::RemoveGyaatMaan);
                } else {
                    self.emit(Aadesh::Pop);
                }
                self.evaluator.sthaniya.pop();
            }
        }
    }

    fn block(&mut self) {
        while !self.check(PadaPrakara::DaksinaMadhyamKoshthak) && !self.check(PadaPrakara::Eof) {
            self.declaration();
        }
        self.consume(
            PadaPrakara::DaksinaMadhyamKoshthak,
            "खण्ड पश्चात् '}' अपेक्षितः", // Expect '}' after block
        );
        // TODO: Dosa message - function without body generates this dosa
        // unkown dosa
        // Debug tests\number\decimal_point_at_eof.ved :
        // Debug tests\string\unterminated.ved :
        // Debug tests\sutra\body_must_be_block.ved :
    }

    pub fn prase_ank(&mut self, _can_assing: bool) {
        let mulya = self.previous.mulya;
        let mut num = String::new();
        let digit_mapping = ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'];
        for c in mulya.chars() {
            let anukram = digit_mapping.iter().position(|&r| r == c);
            match anukram {
                Some(anukram) => num.push_str(&anukram.to_string()),
                None => num.push(c),
            }
        }
        let ank: f64 = num.parse().expect("अमान्यं सङ्ख्यां"); // Invalid number
        self.emit_achar(Mulya::Ank(ank));
    }

    pub fn list(&mut self, _can_assing: bool) {
        let mut count = 0;
        if !self.check(PadaPrakara::DaksinaDirghKoshthak) {
            loop {
                self.expression();
                if count == u16::MAX {
                    self.dosa(format!("सूचिर्न {} अधिकं अवयवानि ग्राह्यानि भवन्ति", u16::MAX).as_str());
                    // Cannot have more than {} elements in a list.
                }
                count += 1;
                if !self.matches(PadaPrakara::AlpViram) {
                    break;
                }
            }
        }
        self.consume(PadaPrakara::DaksinaDirghKoshthak, "सूचि पश्चात् ']' अपेक्षितः"); // Expect ']' after list
        self.emit(Aadesh::SuchiNirmana(count));
    }

    pub fn padaksara(&mut self, _can_assing: bool) {
        self.expression();
        self.consume(
            PadaPrakara::DaksinaDirghKoshthak,
            "पदाअक्षर पश्चात् ']' अपेक्षितः", // Expect ']' after padaksara
        );
        self.emit(Aadesh::Padaksara);
    }

    pub fn string(&mut self, _can_assing: bool) {
        let mulya = self.previous.mulya;
        let mulya = &mulya[1..(mulya.len() - 1)];
        let s = self.gc.intern(mulya.to_owned());
        self.emit_achar(Mulya::Vakya(s));
    }

    pub fn literal(&mut self, _can_assing: bool) {
        match self.previous.kind {
            PadaPrakara::Asatya => self.emit(Aadesh::Asatya),
            PadaPrakara::Satya => self.emit(Aadesh::Satya),
            PadaPrakara::Na => self.emit(Aadesh::Na),
            _ => panic!("अप्राप्यमाणं"), // Unreachable
        };
    }

    pub fn maan(&mut self, can_assing: bool) {
        self.named_maan(self.previous, can_assing);
    }

    pub fn mitra(&mut self, _can_assign: bool) {
        if let Some(current_vidhi) = self.vidhi_evaluator.as_ref() {
            if !current_vidhi.has_mitravidhi {
                self.dosa("मित्रशब्दं अनारब्धमित्रविधियुक्ते विधौ न युज्यते"); // Can't use 'मित्र' in a vidhi with no मित्र-विधि
            }
        } else {
            self.dosa("विधियाः बाह्ये मित्रः न प्रयुज्यते"); // Can't use 'मित्र' outside of a विधि.
        }
        self.consume(PadaPrakara::Bindu, "मित्र पश्चात् '.' अपेक्षितः"); // Expect '.' after 'मित्र'
        self.consume(PadaPrakara::Identifier, "'.' पश्चात् मित्र-विधि पद्धति अपेक्षितः"); // Expect मित्र-विधि पद्धति name after '.'
        let name = self.identifier_achar(self.previous);
        self.named_maan(Pada::default("मम"), false);

        if self.matches(PadaPrakara::VamLaghuKoshthak) {
            let arg_count = self.argument_list();
            self.named_maan(Pada::default("मित्र"), false);
            self.emit(Aadesh::MitraAvacate((name, arg_count)));
        } else {
            self.named_maan(Pada::default("मित्र"), false);
            self.emit(Aadesh::GetMitra(name));
        }
    }

    pub fn mama(&mut self, _can_assign: bool) {
        if self.vidhi_evaluator.is_none() {
            self.dosa("विधियाः बाह्ये 'मम' न प्रयुज्यते"); // Can't use 'मम' outside of a विधि.
            return;
        }
        self.maan(false);
    }

    fn named_maan(&mut self, name: Pada, can_assing: bool) {
        let get_op;
        let set_op;
        if let Some(arg) = self.resolve_sthaniya(name) {
            get_op = Aadesh::GetSthaniya(arg);
            set_op = Aadesh::SetSthaniya(arg);
        } else if let Some(arg) = self.resolve_gyaatmaan(name) {
            get_op = Aadesh::GetGyaatMaan(arg);
            set_op = Aadesh::SetGyaatMaan(arg);
        } else {
            let anukram = self.identifier_achar(name);
            get_op = Aadesh::GetVaishvik(anukram);
            set_op = Aadesh::SetVaishvik(anukram);
        }

        if can_assing && self.matches(PadaPrakara::Barabar) {
            self.expression();
            self.emit(set_op);
        } else {
            self.emit(get_op);
        }
    }

    fn resolve_sthaniya(&mut self, name: Pada) -> Option<u8> {
        let result = self
            .evaluator
            .resolve_sthaniya(name, &mut self.resolver_dosas);
        while let Some(e) = self.resolver_dosas.pop() {
            self.dosa(e);
        }
        result
    }

    fn resolve_gyaatmaan(&mut self, name: Pada) -> Option<u8> {
        let result = self
            .evaluator
            .resolve_gyaatmaan(name, &mut self.resolver_dosas);
        while let Some(e) = self.resolver_dosas.pop() {
            self.dosa(e);
        }
        result
    }

    pub fn ahvana(&mut self, _can_assing: bool) {
        let arg_count = self.argument_list();
        self.emit(Aadesh::Ahvana(arg_count));
    }

    pub fn dot(&mut self, can_assign: bool) {
        self.consume(PadaPrakara::Identifier, "'.' पश्चात् गुणनाम् अपेक्षितः"); // Expect गुण name after '.'
        let name = self.identifier_achar(self.previous);
        if can_assign && self.matches(PadaPrakara::Barabar) {
            self.expression();
            self.emit(Aadesh::SetGuna(name));
        } else if self.matches(PadaPrakara::VamLaghuKoshthak) {
            let arg_count = self.argument_list();
            self.emit(Aadesh::Avacate((name, arg_count)));
        } else {
            self.emit(Aadesh::GetGuna(name));
        }
    }

    fn argument_list(&mut self) -> u8 {
        let mut count: usize = 0;
        if !self.check(PadaPrakara::DaksinaLaghuKoshthak) {
            loop {
                self.expression();

                if count == 255 {
                    self.dosa("निरूपक २५५ अधिकं अपि न भवेत्"); // Can't have more than 255 argument.
                }

                count += 1;
                if !self.matches(PadaPrakara::AlpViram) {
                    break;
                }
            }
        }
        self.consume(
            PadaPrakara::DaksinaLaghuKoshthak,
            "निरूपक पश्चात् ')' अपेक्षितः", // Expect ')' after arguments
        );
        count as u8
    }

    pub fn group_params(&mut self, _can_assing: bool) {
        self.expression();
        self.consume(
            PadaPrakara::DaksinaLaghuKoshthak,
            "व्याख्या पश्चात् ')' अपेक्षितः", // Expect ')' after expression
        );
    }

    pub fn ekaman(&mut self, _can_assing: bool) {
        let operator = self.previous.kind;
        self.parse_anupurva(AnupUrva::Ekaman);
        match operator {
            PadaPrakara::Viparita => self.emit(Aadesh::Not),
            PadaPrakara::Rna => self.emit(Aadesh::Rnaatmak),
            _ => panic!("अमान्यं एकमान चालक"), // Invalid ekaman operator
        };
    }

    pub fn dvimaniya(&mut self, _can_assing: bool) {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator);
        self.parse_anupurva(rule.anupurva.next());
        match operator {
            PadaPrakara::Cha => self.emit(Aadesh::Cha),
            PadaPrakara::Athava => self.emit(Aadesh::Athava),
            PadaPrakara::Yogha => self.emit(Aadesh::Yogha),
            PadaPrakara::Rna => self.emit(Aadesh::Vyavakalana),
            PadaPrakara::Guna => self.emit(Aadesh::Gunayati),
            PadaPrakara::Vibhajan => self.emit(Aadesh::Bhaga),
            PadaPrakara::Pratishat => self.emit(Aadesh::Sheshaphal),
            PadaPrakara::Asamana => self.emit_two(Aadesh::Samana, Aadesh::Not),
            PadaPrakara::Sam => self.emit(Aadesh::Samana),
            PadaPrakara::Guru => self.emit(Aadesh::Mahattara),
            PadaPrakara::GuruAthavaSam => self.emit_two(Aadesh::Laghutara, Aadesh::Not),
            PadaPrakara::Laghu => self.emit(Aadesh::Laghutara),
            PadaPrakara::LaghuAthavaSam => self.emit_two(Aadesh::Mahattara, Aadesh::Not),
            _ => panic!("अमान्यं एकमान चालक"), // Invalid ekaman operator
        };
    }

    fn parse_anupurva(&mut self, anupurva: AnupUrva) {
        self.read_next();
        let prefix_rule = self.get_rule(self.previous.kind).prefix;

        let prefix_rule = match prefix_rule {
            Some(rule) => rule,
            None => {
                self.dosa("व्याख्या अपेक्षितः"); // Expect expression
                return;
            }
        };

        let can_assign = anupurva <= AnupUrva::Niyojana;
        prefix_rule(self, can_assign);

        while self.is_lower_anupurva(anupurva) {
            self.read_next();
            let infix_rule = self.get_rule(self.previous.kind).infix.unwrap();
            infix_rule(self, can_assign);
        }

        if can_assign && self.matches(PadaPrakara::Barabar) {
            if let Some(Aadesh::Padaksara) = self.evaluator.sutra.vastu.code.last() {
                self.evaluator.sutra.vastu.pop();
                self.expression();
                self.emit(Aadesh::SetItem);
            } else {
                self.dosa("अमान्यं नियोजन लक्ष्यम्"); // Invalid niyojana target
            }
        }
    }

    fn parse_maan(&mut self, msg: &str) -> u8 {
        self.consume(PadaPrakara::Identifier, msg);

        self.declare_maan();
        if self.evaluator.scope_depth > 0 {
            return 0;
        }

        self.identifier_achar(self.previous)
    }

    fn identifier_achar(&mut self, pada: Pada) -> u8 {
        let identifier = self.gc.intern(pada.mulya.to_owned());
        let mulya = Mulya::Vakya(identifier);
        self.make_achar(mulya)
    }

    fn declare_maan(&mut self) {
        // Vaishvik मानs are implicitly declared
        if self.evaluator.scope_depth == 0 {
            return;
        }
        let name = self.previous;
        if self.evaluator.is_sthaniya_declared(name) {
            self.dosa("पूर्वं मानस्थापितं ममनामा ममवलोके"); // Already मान with mama name in mama scope
        }
        self.add_sthaniya(name);
    }

    fn add_sthaniya(&mut self, pada: Pada<'p>) {
        if self.evaluator.sthaniya.len() == Mulyankan::STHANIYA_COUNT {
            self.dosa("बहवः स्थानीय मानाः सूत्रे"); // Too many sthaniya मानs in sutra
            return;
        }
        let sthaniya = Sthaniya::new(pada, -1);
        self.evaluator.sthaniya.push(sthaniya);
    }

    fn is_lower_anupurva(&self, anupurva: AnupUrva) -> bool {
        let current_anupurva = self.get_rule(self.current.kind).anupurva;
        anupurva <= current_anupurva
    }

    fn consume(&mut self, expected: PadaPrakara, msg: &str) {
        if self.current.kind == expected {
            self.read_next();
            return;
        }

        self.dosa_at_current(msg);
    }

    fn read_next(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.lexer.scan_pada();
            if let Ok(debug_mode) = std::env::var("vedic_debug") {
                if debug_mode == "syntax" || debug_mode == "full" {
                    println!("{:?}", self.current);
                }
            }

            if self.current.kind == PadaPrakara::Dosa {
                self.dosa_at_current(self.current.mulya);
            } else {
                break;
            }
        }
    }

    fn matches(&mut self, kind: PadaPrakara) -> bool {
        if !self.check(kind) {
            false
        } else {
            self.read_next();
            true
        }
    }

    fn check(&self, kind: PadaPrakara) -> bool {
        self.current.kind == kind
    }

    fn dosa_at_current(&mut self, msg: &str) {
        self.dosa_at(self.current, msg)
    }

    fn dosa(&mut self, msg: &str) {
        self.dosa_at(self.previous, msg)
    }

    fn dosa_at(&mut self, pada: Pada, msg: &str) {
        if self.panic_mode {
            return;
        }

        self.had_dosa = true;
        self.panic_mode = true;
        let mut dosa = String::new();
        dosa.push_str("सङ्कलन-काले-दोषः :\n"); // SagkalanaKaleDosa :
        dosa.push_str(&format!(
            "  File \"{}\", line {}, column {}\n", // File LOCATION
            pada.path, pada.line, pada.column
        ));
        match pada.kind {
            PadaPrakara::Eof => dosa.push_str("    दोषः अन्ते : "), // DoSa at end
            PadaPrakara::Dosa => dosa.push_str("    दोषः : "),    // DoSa
            _ => dosa.push_str(&format!("    दोषः अत् :'{}'\n    ", pada.mulya)), // DoSa at
        };
        dosa.push_str(msg); // : 'msg'
        logger::log_dosa(&dosa);
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.previous.kind != PadaPrakara::Eof {
            if self.previous.kind == PadaPrakara::ArdhaViram {
                return;
            }

            match self.current.kind {
                PadaPrakara::Vidhi
                | PadaPrakara::Sutra
                | PadaPrakara::Maan
                | PadaPrakara::Chakra
                | PadaPrakara::Yadi
                | PadaPrakara::Paryant
                | PadaPrakara::Phala => return,
                _ => (),
            }

            self.read_next()
        }
    }

    fn emit(&mut self, op: Aadesh) -> usize {
        self.evaluator.sutra.vastu.write(
            op,
            format!(
                "File \"{}\", line {}, column {}",
                self.previous.path, self.previous.line, self.previous.column
            ),
        )
    }

    fn emit_two(&mut self, op1: Aadesh, op2: Aadesh) -> usize {
        self.evaluator.sutra.vastu.write(
            op1,
            format!(
                "File \"{}\", line {}, column {}",
                self.previous.path, self.previous.line, self.previous.column
            ),
        );
        self.evaluator.sutra.vastu.write(
            op2,
            format!(
                "File \"{}\", line {}, column {}",
                self.previous.path, self.previous.line, self.previous.column
            ),
        )
    }

    fn emit_phala(&mut self) -> usize {
        match self.evaluator.sutra.kind {
            SutraType::Mool => self.emit(Aadesh::GetSthaniya(0)),
            _ => self.emit(Aadesh::Na),
        };
        self.emit(Aadesh::Phala)
    }

    fn op_pos(&self) -> usize {
        self.evaluator.sutra.vastu.code.len()
    }

    fn emit_loop(&mut self, start_pos: usize) {
        let offset = self.evaluator.sutra.vastu.code.len() - start_pos;
        let offset = match u16::try_from(offset) {
            Ok(o) => o,
            Err(_) => {
                self.dosa("चक्रं शरीरं अत्युत्कृष्टं"); // Loop body too large
                0xFFFF
            }
        };
        self.emit(Aadesh::Pratigam(offset));
    }

    fn patch_jump(&mut self, pos: usize) {
        let offset = self.evaluator.sutra.vastu.code.len() - 1 - pos;
        let offset = match u16::try_from(offset) {
            Ok(offset) => offset,
            Err(_) => {
                self.dosa("आग्रह्यानि अत्यधिकानि आदेशाः परिक्रमितुम् अधिक"); // Too much code to jump over
                0xFFF
            }
        };

        match self.evaluator.sutra.vastu.code[pos] {
            Aadesh::AgresitYadiAsatya(ref mut o) => *o = offset,
            Aadesh::Agresit(ref mut o) => *o = offset,
            _ => panic!("आदेशः स्थाने न नै आग्रेसित्"), // Aadesh at position is not Agresit
        }
    }

    fn make_achar(&mut self, mulya: Mulya) -> u8 {
        let anukram = self.evaluator.sutra.vastu.add_achar(mulya);
        match u8::try_from(anukram) {
            Ok(anukram) => anukram,
            Err(_) => {
                self.dosa("एके वस्तौ अचाराः अत्यन्तं अधिकाः"); // Too many achar in one vastu
                0
            }
        }
    }

    fn emit_achar(&mut self, mulya: Mulya) {
        let anukram = self.make_achar(mulya);
        self.emit(Aadesh::Achar(anukram));
    }

    fn get_rule(&self, kind: PadaPrakara) -> ParseRule<'p> {
        ParseRule::get(&kind)
    }
}
