use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Character {
    AK,
    WB,
    ST,
    General,
}
impl Default for Character {
    fn default() -> Self {
        Self::General
    }
}
impl TryInto<Character> for &str {
    type Error = ();
    fn try_into(self) -> Result<Character, Self::Error> {
        match self {
            "AK" => Ok(Character::AK),
            "WB" => Ok(Character::WB),
            "ST" => Ok(Character::ST),
            _ => Err(()),
        }
    }
}
impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AK => write!(f, "Arthur Kipps"),
            Self::WB => write!(f, "The Woman in Black"),
            Self::ST => write!(f, "Stella"),
            Self::General => write!(f, "General"),
        }
    }
}
