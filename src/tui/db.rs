use english_quotes::{
    db::{get_quote as raw_get_quote, remove_quote},
    quote::Quote,
    utils::Error,
};
use tui::widgets::ListState;

pub fn get_quote(
    category_state: &mut ListState,
    item_state: &mut ListState,
) -> Result<Quote, Error> {
    raw_get_quote(
        category_state.selected().expect("quote type selected"),
        item_state.selected().unwrap_or_default(),
        None,
    )
}

pub fn remove_quote_by_quote(list_state: &mut ListState, q: &Quote) -> Result<(), Error> {
    if let Some(selected) = list_state.selected() {
        remove_quote(q, None)?;
        if selected != 0 {
            list_state.select(Some(selected - 1));
        }
    }

    Ok(())
}
