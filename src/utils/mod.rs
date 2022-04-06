pub mod either;
pub mod events;
pub mod exports;
pub mod render;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Home,
    Quotes,
    Entry,
    QuoteCategory,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::Quotes => 1,
            MenuItem::Entry => 2,
            MenuItem::QuoteCategory => 4,
        }
    }
}
