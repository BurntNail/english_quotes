use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum QuoteType {
    //Characters
    Arthur_Kipps,
    Woman_In_Black,
    Stella,
    //Themes
    Women,
    Gothic_Horror,
    //Other
    Other
}
impl Default for QuoteType {
    fn default() -> Self {
        Self::Other
    }
}
impl Display for QuoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arthur_Kipps => write!(f, "Character: Arthur Kipps"),
            Self::Woman_In_Black => write!(f, "Character: The Woman in Black"),
            Self::Stella => write!(f, "Character: Stella"),
            Self::Women => write!(f, "Theme: Women"),
            Self::Gothic_Horror => write!(f, "Theme: Gothic Horror"),
            Self::Other => write!(f, "General"),
        }
    }
}
