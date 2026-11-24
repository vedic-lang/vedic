use super::fetchsource::Sourcecode;
use super::pada::Pada;
use super::pada::PadaPrakara;
pub struct Lexer<'s> {
    sourcecode: &'s Sourcecode<'s>,
    code_vec: Vec<char>,
    start: usize,
    at: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(sourcecode: &'s Sourcecode) -> Lexer<'s> {
        Lexer {
            sourcecode,
            code_vec: sourcecode.code.chars().collect(),
            start: 0,
            at: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_pada(&mut self) -> Pada<'s> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_eof() {
            return self.make_pada(PadaPrakara::Eof);
        }

        match self.read_next() {
            '(' => self.make_pada(PadaPrakara::VamLaghuKoshthak),
            ')' => self.make_pada(PadaPrakara::DaksinaLaghuKoshthak),
            '{' => self.make_pada(PadaPrakara::VamMadhyamKoshthak),
            '}' => self.make_pada(PadaPrakara::DaksinaMadhyamKoshthak),
            '[' => self.make_pada(PadaPrakara::VamDirghKoshthak),
            ']' => self.make_pada(PadaPrakara::DaksinaDirghKoshthak),
            ';' => self.make_pada(PadaPrakara::ArdhaViram),
            ',' => self.make_pada(PadaPrakara::AlpViram),
            '.' => self.make_pada(PadaPrakara::Bindu),
            '-' => self.make_pada(PadaPrakara::Rna),
            '+' => self.make_pada(PadaPrakara::Yogha),
            '/' => self.make_pada(PadaPrakara::Vibhajan),
            '%' => self.make_pada(PadaPrakara::Pratishat),
            '*' => self.make_pada(PadaPrakara::Guna),
            '&' if self.matches('&') => self.make_pada(PadaPrakara::Cha),
            '|' if self.matches('|') => self.make_pada(PadaPrakara::Athava),
            '!' if self.matches('=') => self.make_pada(PadaPrakara::Asamana),
            '!' => self.make_pada(PadaPrakara::Viparita),
            '=' if self.matches('=') => self.make_pada(PadaPrakara::Sam),
            '=' => self.make_pada(PadaPrakara::Barabar),
            '<' if self.matches('=') => self.make_pada(PadaPrakara::LaghuAthavaSam),
            '<' => self.make_pada(PadaPrakara::Laghu),
            '>' if self.matches('=') => self.make_pada(PadaPrakara::GuruAthavaSam),
            '>' => self.make_pada(PadaPrakara::Guru),
            '\'' => self.read_string('\''),
            '"' => self.read_string('"'),
            c if self.is_devanagari_digit(c) => self.read_number(),
            c if self.is_identifier(c) => self.read_identifier(),
            _ => self.dosa_pada("Unexpected character."),
        }
    }

    fn is_eof(&self) -> bool {
        self.current == self.sourcecode.code.len()
    }

    fn get_mulya(&self) -> &'s str {
        &self.sourcecode.code[self.start..self.current]
    }

    pub(crate) fn byte_offset(&self) -> usize {
        self.current
    }

    // A token's end is a UTF-8 boundary, including the closing quote of a multiline string.
    pub(crate) fn insertion_line(&self, end: usize) -> (&str, &str) {
        let code = &self.sourcecode.code;
        let start = code[..end].rfind('\n').map_or(0, |newline| newline + 1);
        let line_end = code[end..].find('\n').map_or(code.len(), |rest| end + rest);
        let line = &code[start..line_end];
        (line.strip_suffix('\r').unwrap_or(line), &code[start..end])
    }

    fn make_pada(&self, kind: PadaPrakara) -> Pada<'s> {
        Pada {
            kind,
            mulya: self.get_mulya(),
            path: self.sourcecode.path,
            line: self.line,
            column: self.column,
        }
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.code_vec[self.at]
        }
    }

    fn peek_next(&self) -> char {
        self.code_vec
            .get(self.at.saturating_add(1))
            .copied()
            .unwrap_or('\0')
    }

    fn dosa_pada(&self, message: &'static str) -> Pada<'s> {
        Pada {
            kind: PadaPrakara::Dosa,
            mulya: message,
            path: self.sourcecode.path,
            line: self.line,
            column: self.column,
        }
    }

    fn read_next(&mut self) -> char {
        let char = self.peek();
        self.at += 1;
        self.current += char.len_utf8();
        self.column += 1; // char.len_utf8();
        char
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_eof() || self.peek() != expected {
            false
        } else {
            self.at += 1;
            self.current += expected.len_utf8();
            self.column += 1; // expected.len_utf8();
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.read_next();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.read_next();
                }
                '#' => {
                    while self.peek() != '\n' && !self.is_eof() {
                        self.read_next();
                    }
                }
                _ => return,
            }
        }
    }

    fn read_string(&mut self, quotes: char) -> Pada<'s> {
        // TODO: Handle escape sequences
        while self.peek() != quotes && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.read_next();
        }

        if self.is_eof() {
            self.dosa_pada("Unterminated string.")
        } else {
            self.read_next();
            self.make_pada(PadaPrakara::String)
        }
    }

    fn read_number(&mut self) -> Pada<'s> {
        while self.is_devanagari_digit(self.peek()) {
            self.read_next();
        }

        if self.peek() == '.' && self.is_devanagari_digit(self.peek_next()) {
            self.read_next();
            while self.is_devanagari_digit(self.peek()) {
                self.read_next();
            }
        }

        self.make_pada(PadaPrakara::Ank)
    }

    fn read_identifier(&mut self) -> Pada<'s> {
        while self.is_identifier(self.peek()) || self.is_devanagari_digit(self.peek()) {
            self.read_next();
        }
        self.make_pada(self.identifier_type())
    }

    fn identifier_type(&self) -> PadaPrakara {
        match self.get_mulya() {
            "मान" => PadaPrakara::Maan,
            "यदि" => PadaPrakara::Yadi,
            "अथ" => PadaPrakara::Atha,
            "सत्य" => PadaPrakara::Satya,
            "असत्य" => PadaPrakara::Asatya,
            "न" => PadaPrakara::Na,
            "सूत्र" => PadaPrakara::Sutra,
            "फल" => PadaPrakara::Phala,
            "विधि" => PadaPrakara::Vidhi,
            "सन्धि" => PadaPrakara::Sandhi,
            "मित्र" => PadaPrakara::Mitra,
            "मम" => PadaPrakara::Mama,
            "चक्र" => PadaPrakara::Chakra,
            "पर्यन्त" => PadaPrakara::Paryant,
            "विराम" => PadaPrakara::Viram,
            "अग्रिम" => PadaPrakara::Agrim,
            "निर्देश" => PadaPrakara::Nirdesh,
            "आवाहन" => PadaPrakara::Import,
            _ => PadaPrakara::Identifier,
        }
    }

    fn is_identifier(&self, c: char) -> bool {
        c.is_alphabetic()
            || c == '_'
            || ('\u{0900}'..='\u{0965}').contains(&c)
            || ('\u{0970}'..='\u{097F}').contains(&c)
    }

    fn is_devanagari_digit(&self, c: char) -> bool {
        ('\u{0966}'..='\u{096F}').contains(&c) || c.is_ascii_digit()
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::PadaPrakara;
    use crate::fetchsource::Sourcecode;

    #[test]
    fn trailing_decimal_point_is_scanned_without_panicking() {
        let sourcecode = Sourcecode {
            code: "१.".to_owned(),
            path: "test",
        };
        let mut lexer = Lexer::new(&sourcecode);

        assert_eq!(lexer.scan_pada().kind, PadaPrakara::Ank);
        assert_eq!(lexer.scan_pada().kind, PadaPrakara::Bindu);
        assert_eq!(lexer.scan_pada().kind, PadaPrakara::Eof);
    }

    #[test]
    fn devanagari_decimal_scans_as_one_number() {
        let sourcecode = Sourcecode {
            code: "१२.३".to_owned(),
            path: "test",
        };
        let mut lexer = Lexer::new(&sourcecode);

        let number = lexer.scan_pada();
        assert_eq!(number.kind, PadaPrakara::Ank);
        assert_eq!(number.mulya, "१२.३");
        assert_eq!(lexer.scan_pada().kind, PadaPrakara::Eof);
    }
}
