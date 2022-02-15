use crate::{Character, Character::*, Theme, Theme::*};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub Either<Character, Theme>);

const fn l(lhs: Character) -> Either<Character, Theme> {
    Either::Left(lhs)
}
const fn r(lhs: Theme) -> Either<Character, Theme> {
    Either::Right(lhs)
}

pub const ALL_PERMS: &[Either<Character, Theme>] = &[l(AK), l(WB), l(ST), r(WO), r(GH)];
