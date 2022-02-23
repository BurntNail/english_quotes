use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum QuoteType {
    //Characters
    ArthurKipps,
    WomanInBlack,
    Stella,
    Esme,
    Isabel,
    Spider,
    MrBently,
    SamuelDaily,
    MrsDaily,
    Drablow,
    //Themes
    Women,
    GothicHorror,
    //Locations
    CrythinGifford,
    DailyManor,
    GiffordArms,
    //Other
    Other,
}

use QuoteType::*;

impl Default for QuoteType {
    fn default() -> Self {
        Other
    }
}
impl Display for QuoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArthurKipps => write!(f, "Character: Arthur Kipps"),
            WomanInBlack => write!(f, "Character: The Woman in Black"),
            Stella => write!(f, "Character: Stella"),
            Esme => write!(f, "Character: Esme"),
            Isabel => write!(f, "Character: Isabel"),
            Spider => write!(f, "Character: Spider"),
            MrBently => write!(f, "Character: Mr Bently"),
            SamuelDaily => write!(f, "Character: Samuel Daily"),
            MrsDaily => write!(f, "Character: Mrs Daily"),
            Drablow => write!(f, "Character: Mrs Drablow"),
            Women => write!(f, "Theme: Women"),
            GothicHorror => write!(f, "Theme: Gothic Horror"),
            CrythinGifford => write!(f, "Location: Crythin Gifford"),
            DailyManor => write!(f, "Location: The Daily Manor"),
            GiffordArms => write!(f, "Location: The Gifford Arms"),
            Other => write!(f, "General"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub Vec<QuoteType>);

pub const ALL_PERMS: &[QuoteType] = &[
    ArthurKipps,
    WomanInBlack,
    Stella,
    Esme,
    Isabel,
    Spider,
    MrBently,
    SamuelDaily,
    MrsDaily,
    Drablow,
    Women,
    GothicHorror,
    CrythinGifford,
    DailyManor,
    GiffordArms,
    Other,
];

impl PartialEq<&Quote> for Quote {
    fn eq(&self, other: &&Quote) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}
