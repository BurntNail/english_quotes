use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

lazy_static! {
    pub static ref ALL_PERMS: Vec<String> = {
        let location = FileType::Types.get_location();
        std::fs::read_to_string(&location)
            .expect(&format!("Could not find t.t at {}", location))
            .split('\n')
            .filter(|ty| !ty.contains("//"))
        .map(|ty| {
            if ty.contains('\r') {
                let len = ty.len();
                &ty[..len-1]
            } else {
                ty
            }.to_string()
        })
            .collect()
    };
}


pub enum FileType {
    Database,
    Types,
    Export
}

impl FileType {
    pub fn get_location (&self) -> &'static str {
        match self {
            Self::Database => "db.json",
            Self::Types => "types.txt",
            Self::Export => "export.md",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote(pub String, pub Vec<String>);

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

impl PartialEq<&Self> for Quote {
    fn eq(&self, other: &&Self) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}
