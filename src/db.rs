use crate::{
    quote::{Quote, ALL_PERMS},
    utils::Error,
};
use std::fs::read_to_string;
use tui::widgets::ListState;

const DB_PATH: &str = "./db.json";

pub fn add_quote_to_db(q: Quote) -> Result<Vec<Quote>, Error> {
    let db_content = read_to_string(DB_PATH).unwrap_or_default();
    let mut parsed: Vec<Quote> = serde_json::from_str(&db_content).unwrap_or_default();

    parsed.push(q);
    std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

pub fn remove_quote_by_quote(list_state: &mut ListState, q: Quote) -> Result<(), Error> {
    if let Some(selected) = list_state.selected() {
        let db_contents = read_to_string(DB_PATH)?;
        let mut parsed: Vec<Quote> = serde_json::from_str(&db_contents)?;
        let pos = parsed.iter().position(|q_loco| q == q_loco).unwrap();
        parsed.remove(pos);
        std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;

        if selected != 0 {
            list_state.select(Some(selected - 1));
        }
    }

    Ok(())
}

pub fn read_db() -> Result<Vec<Quote>, Error> {
    let db_content = read_to_string(DB_PATH).unwrap_or_else(|_| "[]".into());
    let parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn get_quote(category_state: &mut ListState, item_state: &mut ListState) -> (Quote, usize) {
    let quote_type_index = category_state.selected().expect("quote type selected");
    let db = read_db().expect("can read db");

    let q = ALL_PERMS[quote_type_index];

    let quote = db
        .into_iter()
        .filter(|quote| quote.1 == q)
        .nth(item_state.selected().unwrap_or_default())
        .unwrap();

    (quote, quote_type_index)
}
