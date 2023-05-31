#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum PadaPrakara {
    // Symbols
    AlpViram,               // ,
    ArdhaViram,             // ;
    Asamana,                // !=
    Athava,                 // ||
    Barabar,                // =
    Bindu,                  // .
    Cha,                    // &&
    DaksinaDirghKoshthak,   // ]
    DaksinaLaghuKoshthak,   // )
    DaksinaMadhyamKoshthak, // }
    Guna,                   // *
    Guru,                   // >
    GuruAthavaSam,          // >=
    Laghu,                  // <
    LaghuAthavaSam,         // <=
    Pratishat,              // %
    Rna,                    // -
    Sam,                    // ==
    VamDirghKoshthak,       // [
    VamLaghuKoshthak,       // (
    VamMadhyamKoshthak,     // {
    Vibhajan,               // /
    Viparita,               // !
    Yogha,                  // +

    // Literals.
    Ank,        // Number
    Identifier, // Identifier
    String,     // String

    // Kunjiwords.
    Agrim,   // Continue
    Asatya,  // False
    Atha,    // Else
    Chakra,  // For loop
    Import,  // Import
    Maan,    // variable
    Mama,    // this or self
    Mitra,   // super or parent class avastha variable
    Na,      // Null
    Nirdesh, // Switch
    Paryant, // While loop
    Phala,   // Return
    Sandhi,  // Inheritance
    Satya,   // True
    Sutra,   // Function
    Vidhi,   // Class
    Viram,   // Break
    Yadi,    // IF

    // Dosas.
    Dosa,
    Eof,
}

#[derive(Copy, Clone, Debug)]
pub struct Pada<'s> {
    pub kind: PadaPrakara,
    pub path: &'s str,
    pub line: usize,
    pub column: usize,
    pub mulya: &'s str,
}

impl<'s> Pada<'s> {
    pub fn default(text: &'s str) -> Pada<'s> {
        Pada {
            kind: PadaPrakara::Dosa,
            mulya: text,
            path: "<unknown>",
            line: 0,
            column: 0,
        }
    }
}
