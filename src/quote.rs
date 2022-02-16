use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use crate::quote_type::{QuoteType, QuoteType::*};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub QuoteType);

pub const ALL_PERMS: &[QuoteType] = &[Arthur_Kipps, Woman_In_Black, Stella, Women, Gothic_Horror, Other];

impl From<&str> for Quote {
    fn from(input: &str) -> Self {
        let space_pos = input.chars().position(|c| c == ' ').unwrap();
        let code: &QuoteType = &input[1..space_pos].try_into().unwrap_or_default();
        let actual_contents = &input[space_pos + 1..];
        
        Quote(actual_contents.to_string(), *code)
    }
}
