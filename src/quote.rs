use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum QuoteType {
    //Characters
    ArthurKipps,
    WomanInBlack,
    Stella,
    Esme,
    SamuelDaily,
    Drablow,
    //Themes
    Women,
    GothicHorror,
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
            SamuelDaily => write!(f, "Character: Samuel Daily"),
            Drablow => write!(f, "Character: Mrs Drablow"),
            Women => write!(f, "Theme: Women"),
            GothicHorror => write!(f, "Theme: Gothic Horror"),
            Other => write!(f, "General"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub QuoteType);

pub const ALL_PERMS: &[QuoteType] = &[
    ArthurKipps,
    WomanInBlack,
    Stella,
    Esme,
    SamuelDaily,
    Drablow,
    Women,
    GothicHorror,
    Other,
];

impl PartialEq<&Quote> for Quote {
    fn eq(&self, other: &&Quote) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}