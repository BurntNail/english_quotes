use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use enum_derive::AllVariants;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, Copy, Clone, AllVariants)]
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
    Nathaniel,
    CGPeople,
    //Themes
    Women,
    GothicHorror,
    Secrecy,
    Innocence,
    //Locations
    CrythinGifford,
    DailyManor,
    GiffordArms,
    EelMarshHouse,
    MonksPiece,
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
            Nathaniel => write!(f, "Character: Nathaniel"),
            CGPeople => write!(f, "Characters: The People of Crythin Gifford"),
            Women => write!(f, "Theme: Women"),
            GothicHorror => write!(f, "Theme: Gothic Horror"),
            Innocence => write!(f, "Theme: Innocence"),
            Secrecy => write!(f, "Theme: Mystery & Secrets"),
            CrythinGifford => write!(f, "Location: Crythin Gifford"),
            DailyManor => write!(f, "Location: The Daily Manor"),
            GiffordArms => write!(f, "Location: The Gifford Arms"),
            EelMarshHouse => write!(f, "Location: Eel Marsh House"),
            MonksPiece => write!(f, "Location: Monk's Piece"),
            Other => write!(f, "General"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub Vec<QuoteType>);

pub const ALL_PERMS: &[QuoteType] = QuoteType::all_variants();

impl Eq for Quote {}

impl PartialEq for Quote {
    fn eq(&self, other: &Self) -> bool {
        let mut l1 = self.1.clone();
        let mut l2 = other.1.clone();

        l1.sort();
        l2.sort();

        (self.0 == other.0) && (l1 == l2)

    }
}

impl Ord for Quote {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut l1 = self.1.clone();
        let mut l2 = other.1.clone();

        l1.sort();
        l2.sort();

        for (i, el) in l1.into_iter().enumerate() {
            if let Some(l2_el) = l2.get(i) {
                let c = el.cmp(l2_el);
                if c != Ordering::Equal {
                    return c;
                }
            } else {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Quote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<&Quote> for Quote {
    fn eq(&self, other: &&Quote) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}
