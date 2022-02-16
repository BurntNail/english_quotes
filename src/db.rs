use crate::quote::{Quote, ALL_PERMS};
use crate::util::Error;
use std::fs::read_to_string;
use tui::widgets::ListState;
use rand::Rng;

const DB_PATH: &str = "./db.json";

pub fn add_quote_to_db(q: Quote) -> Result<Vec<Quote>, Error> {
	let db_content = read_to_string(DB_PATH)?;
	let mut parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
	
	parsed.push(q);
	std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
	Ok(parsed)
}

pub fn remove_quote_at_index(quote_list_state: &mut ListState) -> Result<(), Error> {
	if let Some(selected) = quote_list_state.selected() {
		let db_contents = read_to_string(DB_PATH)?;
		let mut parsed: Vec<Quote> = serde_json::from_str(&db_contents)?;
		parsed.remove(selected);
		std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
		
		if selected != 0 {
			quote_list_state.select(Some(selected - 1));
		}
	}
	
	Ok(())
}

#[allow(dead_code)]
pub fn add_random_quote() -> Result<Vec<Quote>, Error> {
	let mut rng = rand::thread_rng();
	let db_content = read_to_string(DB_PATH)?;
	let mut parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
	
	let contents: i128 = rng.gen();
	let tt = ALL_PERMS[rng.gen_range(0..ALL_PERMS.len())];
	let rnd_quote = Quote(contents.to_string(), tt);
	
	parsed.push(rnd_quote);
	std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
	
	Ok(parsed)
}

pub fn read_db() -> Result<Vec<Quote>, Error> {
	let db_content = read_to_string(DB_PATH)?;
	let parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
	Ok(parsed)
}