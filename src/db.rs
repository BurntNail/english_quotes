use crate::{
    quote::{FileType, Quote, ALL_PERMS},
    utils::Error,
};
use std::fs::read_to_string;

pub fn add_quote_to_db(mut q: Quote) -> Result<Vec<Quote>, Error> {
    let db_content = read_to_string(FileType::Database.get_location()).unwrap_or_default();
    let mut parsed: Vec<Quote> = serde_json::from_str(&db_content).unwrap_or_default();

    if q.1.is_empty() {
        q.1.push("Other".into());
    }

    parsed.push(q);
    std::fs::write(
        FileType::Database.get_location(),
        &serde_json::to_vec(&parsed)?,
    )?;
    Ok(parsed)
}

pub fn remove_quote(q: &Quote) -> Result<(), Error> {
    let db_contents = read_to_string(FileType::Database.get_location())?;
    let mut parsed: Vec<Quote> = serde_json::from_str(&db_contents)?;

    if let Some(pos) = parsed.iter().position(|q_loco| q == q_loco) {
        parsed.remove(pos);

        std::fs::write(
            FileType::Database.get_location(),
            &serde_json::to_vec(&parsed)?,
        )?;
    } else {
        return Err(Error::QuoteNotFoundInDB(q.clone()));
    }

    Ok(())
}

pub fn read_db() -> Result<Vec<Quote>, Error> {
    let db_content =
        read_to_string(FileType::Database.get_location()).unwrap_or_else(|_| "[]".into());
    let parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn get_quote(category_index: usize, item_index: usize) -> Result<Quote, Error> {
    let db = read_db().expect("can read db");
    let q = ALL_PERMS[category_index].to_string();

    db.into_iter()
        .filter(|quote| quote.1.contains(&q))
        .nth(item_index)
        .ok_or(Error::QuoteNotFoundIndex(category_index, item_index))
}

pub fn get_quote_by_content(content: &str) -> Result<Quote, Error> {
    read_db()
        .unwrap_or_default()
        .into_iter()
        .find(|quote| quote.0 == content)
        .ok_or_else(|| Error::QuoteNotFoundStr(content.to_string()))
}

pub fn sort_list() -> Result<(), Error> {
    let mut db: Vec<_> = read_db()?
        .into_iter()
        .map(|quote| {
            let mut l = quote.1.clone();
            l.sort();

            Quote(quote.0, l)
        })
        .collect();
    db.sort();

    std::fs::write(FileType::Database.get_location(), &serde_json::to_vec(&db)?)?;

    Ok(())
}
