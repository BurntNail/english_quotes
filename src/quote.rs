use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub Vec<String>);

pub const ALL_PERMS: &[&str] = &[
    "Character: Arthur Kipps",
    "Character: The Woman in Black",
    "Character: Stella",
    "Character: Esme",
    "Character: Isabel",
    "Character: Spider",
    "Character: Mr Bently",
    "Character: Samuel Daily",
    "Character: Mrs Daily",
    "Character: Mrs Drablow",
    "Character: Nathaniel",
    "Characters: The People of Crythin Gifford",
    "Theme: Women",
    "Theme: Gothic Horror",
    "Theme: Innocence",
    "Theme: Mystery & Secrets",
    "Theme: Supernatural",
    "Location: Crythin Gifford",
    "Location: The Daily Manor",
    "Location: The Gifford Arms",
    "Location: Eel Marsh House",
    "Location: Monk's Piece",
    "Location: Nine Lives Causway + Marshes",
    "General",
    "Character: Arthur Kipps",
    "Character: The Woman in Black",
    "Character: Stella",
    "Character: Esme",
    "Character: Isabel",
    "Character: Spider",
    "Character: Mr Bently",
    "Character: Samuel Daily",
    "Character: Mrs Daily",
    "Character: Mrs Drablow",
    "Character: Nathaniel",
    "Characters: The People of Crythin Gifford",
    "Theme: Women",
    "Theme: Gothic Horror",
    "Theme: Innocence",
    "Theme: Mystery & Secrets",
    "Theme: Supernatural",
    "Location: Crythin Gifford",
    "Location: The Daily Manor",
    "Location: The Gifford Arms",
    "Location: Eel Marsh House",
    "Location: Monk's Piece",
    "Location: Nine Lives Causway + Marshes",
    "General",
];

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
