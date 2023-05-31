#[derive(Debug, Copy, Clone)]
pub enum Aadesh {
    Cha,                    // And
    Yogha,                  // Addition
    SuchiNirmana(u16),      // Array Creation
    Athava,                 // Or
    Ahvana(u8),             // Call
    Agrim,                  // Continue
    Vidhi(u8),              // Class
    RemoveGyaatMaan,        // Remove GyaatMaan
    Utsarga(u8),            // Closure	value
    Achar(u8),              // Constant
    DefineVaishvik(u8),     // Define Global
    Bhaga,                  // division
    Samana,                 // Equal
    Asatya,                 // False
    GetVaishvik(u8),        // Get Global
    GetSthaniya(u8),        // Get Local
    GetGuna(u8),            // Get Guna
    GetMitra(u8),           // Get Mitra
    GetGyaatMaan(u8),       // Get GyaatMaan
    Mahattara,              // Greater
    Anuharana,              // Inherit
    Avacate((u8, u8)),      // Invoke
    Agresit(u16),           // Jump/GoTo
    AgresitYadiAsatya(u16), // JumpIfFalse
    Laghutara,              // Less
    Pratigam(u16),          // Loop
    Paddhati(u8),           // Method
    Sheshaphal,             // Modulus
    Gunayati,               // Multiply
    Rnaatmak,               // Negative
    Na,                     // Null
    Not,                    // Not (logical)
    Pop,                    // Pop
    Phala,                  // Return
    SetSthaniya(u8),        // Set Local
    SetVaishvik(u8),        // Set Global
    SetGuna(u8),            // Set Property
    SetGyaatMaan(u8),       // Set GyaatMaan
    Padaksara,              // Subscript
    Vyavakalana,            // Subtract
    MitraAvacate((u8, u8)), // SuperInvoke
    Satya,                  // True
    Viraam,                 // Break
    SetItem,                // To List(Suchi)
}
