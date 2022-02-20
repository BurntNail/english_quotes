use crate::{
    db::read_db,
    quote::{Quote, ALL_PERMS},
};
use thiserror::*;
use tui::widgets::ListState;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Home,
    Quotes,
    Entry,
    Quit,
    QuoteCategory,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::Quotes => 1,
            MenuItem::Entry => 2,
            MenuItem::QuoteCategory => 4,
            MenuItem::Quit => 5,
        }
    }
}

pub fn default_state() -> ListState {
    let mut s = ListState::default();
    s.select(Some(0));
    s
}

pub fn down_arrow(state: &mut ListState, amt: usize) {
    if let Some(selected) = state.selected() {
        if selected >= amt - 1 {
            state.select(Some(0));
        } else {
            state.select(Some(selected + 1))
        }
    }
}

pub fn up_arrow(state: &mut ListState, amt: usize) {
    if let Some(selected) = state.selected() {
        if selected > 0 {
            state.select(Some(selected - 1));
        } else {
            state.select(Some(amt - 1))
        }
    }
}

pub fn get_quote(quotes_viewer_main_state: &mut ListState) -> (Quote, usize) {
    let quote_type_index = quotes_viewer_main_state
        .selected()
        .expect("quote type selected");
    let db = read_db().expect("can read db");

    let q = ALL_PERMS[quote_type_index];
    let quote = db
        .into_iter()
        .filter(|quote| quote.1 == q)
        .nth(quotes_viewer_main_state.selected().unwrap_or_default())
        .unwrap();

    (quote, quote_type_index)
}
