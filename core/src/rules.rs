use super::anupurva::AnupUrva;
use super::pada::PadaPrakara;
use super::parser::Parser;

type ParseFn<'p> = fn(&mut Parser<'p>, can_assing: bool) -> ();

pub struct ParseRule<'p> {
    pub prefix: Option<ParseFn<'p>>,
    pub infix: Option<ParseFn<'p>>,
    pub anupurva: AnupUrva,
}

impl<'p> ParseRule<'p> {
    pub fn new(
        prefix: Option<ParseFn<'p>>,
        infix: Option<ParseFn<'p>>,
        anupurva: AnupUrva,
    ) -> ParseRule<'p> {
        Self {
            prefix,
            infix,
            anupurva,
        }
    }

    #[rustfmt::skip]
    pub fn get(kind: &PadaPrakara) -> ParseRule<'p> {
        match kind {
            PadaPrakara::Cha => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Cha),
            PadaPrakara::Ank => Self::new(Some(Parser::prase_ank), None, AnupUrva::Naiva),
            PadaPrakara::Asatya => Self::new(Some(Parser::literal), None, AnupUrva::Naiva),
            PadaPrakara::Vibhajan => Self::new(None, Some(Parser::dvimaniya), AnupUrva::GunaBhaga),
            PadaPrakara::Bindu => Self::new(None, Some(Parser::dot), AnupUrva::Ahvana),
            PadaPrakara::Sam => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Samaanata),
            PadaPrakara::Guru => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Tulana),
            PadaPrakara::GuruAthavaSam => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Tulana),
            PadaPrakara::Identifier => Self::new(Some(Parser::maan), None, AnupUrva::Naiva),
            PadaPrakara::VamLaghuKoshthak => Self::new(Some(Parser::group_params), Some(Parser::ahvana), AnupUrva::Ahvana),
            PadaPrakara::VamDirghKoshthak => Self::new(Some(Parser::list),Some(Parser::padaksara),AnupUrva::Ahvana),
            PadaPrakara::Laghu => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Tulana),
            PadaPrakara::LaghuAthavaSam => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Tulana),
            PadaPrakara::Mama => Self::new(Some(Parser::mama), None, AnupUrva::Naiva),
            PadaPrakara::Rna => Self::new(Some(Parser::ekaman), Some(Parser::dvimaniya), AnupUrva::DhanaRna),
            PadaPrakara::Mitra => Self::new(Some(Parser::mitra), None, AnupUrva::Naiva),
            PadaPrakara::Guna => Self::new(None, Some(Parser::dvimaniya), AnupUrva::GunaBhaga),
            PadaPrakara::Na => Self::new(Some(Parser::literal), None, AnupUrva::Naiva),
            PadaPrakara::Viparita => Self::new(Some(Parser::ekaman), None, AnupUrva::Naiva),
            PadaPrakara::Asamana => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Samaanata),
            PadaPrakara::Athava => Self::new(None, Some(Parser::dvimaniya), AnupUrva::Athava),
            PadaPrakara::Pratishat => Self::new(None, Some(Parser::dvimaniya), AnupUrva::GunaBhaga),
            PadaPrakara::Yogha => Self::new(None, Some(Parser::dvimaniya), AnupUrva::DhanaRna),
            PadaPrakara::Satya => Self::new(Some(Parser::literal), None, AnupUrva::Naiva),
            PadaPrakara::String => Self::new(Some(Parser::string), None, AnupUrva::Naiva),
            _ => Self::new(None, None, AnupUrva::Naiva),
        }
    }
}
