#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum AnupUrva {
    Naiva,     // None
    Niyojana,  // =
    Athava,    // or
    Cha,       // and
    Samaanata, // == !=
    Tulana,    // < > <= >=
    DhanaRna,  // + -
    GunaBhaga, // * / %
    Ekaman,    // ! -(unary minus)
    Ahvana,    // . ()
    Praathamik,
}

impl AnupUrva {
    pub fn next(&self) -> AnupUrva {
        match self {
            AnupUrva::Naiva => AnupUrva::Niyojana,
            AnupUrva::Niyojana => AnupUrva::Athava,
            AnupUrva::Athava => AnupUrva::Cha,
            AnupUrva::Cha => AnupUrva::Samaanata,
            AnupUrva::Samaanata => AnupUrva::Tulana,
            AnupUrva::Tulana => AnupUrva::DhanaRna,
            AnupUrva::DhanaRna => AnupUrva::GunaBhaga,
            AnupUrva::GunaBhaga => AnupUrva::Ekaman,
            AnupUrva::Ekaman => AnupUrva::Ahvana,
            AnupUrva::Ahvana => AnupUrva::Praathamik,
            AnupUrva::Praathamik => AnupUrva::Naiva,
        }
    }
}
