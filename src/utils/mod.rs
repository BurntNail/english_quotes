pub mod either;
pub mod exports;

use crate::quote::Quote;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
    #[error("Error finding quote in DB: {0}")]
    QuoteNotFoundInDB(Quote),
    #[error("Unable to find quote in category {0} index {1}")]
    QuoteNotFoundIndex(usize, usize),
    #[error("Unable to find a quote with content: {0}")]
    QuoteNotFoundStr(String),
}

#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Home,
    Quotes,
    Entry,
    QuoteCategory,
    Find,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::Quotes => 1,
            MenuItem::Entry => 2,
            MenuItem::Find => 3,
            MenuItem::QuoteCategory => 4,
        }
    }
}
