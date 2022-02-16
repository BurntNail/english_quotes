use crate::{Character, Character::*, Theme, Theme::*};
use either::Either;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub Either<Character, Theme>);

const fn l(lhs: Character) -> Either<Character, Theme> {
    Either::Left(lhs)
}
const fn r(lhs: Theme) -> Either<Character, Theme> {
    Either::Right(lhs)
}

pub const ALL_PERMS: &[Either<Character, Theme>] = &[l(AK), l(WB), l(ST), r(WO), r(GH)];

impl From<&str> for Quote {
    fn from(input: &str) -> Self {
        //     std::io::stdin().read_line(&mut input)?;

        //     let input_read = input.trim();

        //     if input_read == "exit" {
        //         break;
        //     }

        //     let space_pos = input_read.chars().position(|c| c == ' ').unwrap();
        //     let is_char = &input_read[..1] == "C";
        //     let code = &input_read[1..space_pos];
        //     let quote = &input_read[space_pos+1..];

        //     let list = if is_char {
        //         chars.entry(code.try_into().unwrap_or_default()).or_default()
        //     } else {
        //         themes.entry(code.try_into().unwrap_or_default()).or_default()
        //     };
        //     list.push(quote.to_string());

        let space_pos = input.chars().position(|c| c == ' ').unwrap();
        let is_char = &input[..1] == "C";
        let code = &input[1..space_pos];
        let actual_contents = &input[space_pos + 1..];

        let acc_code: Either<Character, Theme> = if is_char {
            l(code.try_into().unwrap_or_default())
        } else {
            r(code.try_into().unwrap_or_default())
        };

        Quote(actual_contents.to_string(), acc_code)
    }
}
