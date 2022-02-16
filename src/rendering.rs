use tui::widgets::{Paragraph, Block, Borders, BorderType, ListState, List, Table, ListItem, Row};
use tui::text::{Span, Spans};
use tui::style::{Style, Color, Modifier};
use tui::layout::{Alignment, Constraint};
use crate::quote::ALL_PERMS;
use crate::db::read_db;

fn get_type_items<'a> () -> Vec<ListItem<'a>> {
	ALL_PERMS
		.iter()
		.map(|quote| {
			ListItem::new(Spans::from(vec![Span::styled(
				format!("{}", quote),
				Style::default(),
			)]))
		})
		.collect()
}

pub fn render_home<'a>() -> Paragraph<'a> {
	let home = Paragraph::new(vec![
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::raw("Welcome")]),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::raw("to")]),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::styled(
			"quotes-CLI",
			Style::default().fg(Color::LightBlue),
		)]),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::raw("Press 'q' to access the Quotes, 'e' to enter a new Quote, 'h' to get back home, and 'g' to exit.")]),
	])
		.alignment(Alignment::Center)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.style(Style::default().fg(Color::White))
				.title("Home")
				.border_type(BorderType::Plain),
		);
	home
}

pub fn render_quotes<'a>(quotes_list_state: &ListState) -> (List<'a>, Table<'a>) {
	let quotes = Block::default()
		.borders(Borders::ALL)
		.style(Style::default().fg(Color::White))
		.title("Quotes")
		.border_type(BorderType::Plain);
	
	let quotes_list = read_db().expect("can fetch quotes list");
	
	
	let quote_detail = if !quotes_list.is_empty() {
		let selected_type = ALL_PERMS[quotes_list_state
			.selected()
			.expect("there is always a selected type in the types list")];
		let rows: Vec<_> = quotes_list.into_iter().filter(|quote| quote.1 == selected_type).map(|quote| {
			Row::new(vec![
				Span::raw(format!("{}", quote.1)),
				Span::raw(quote.0)])
		}).collect();
		
		Table::new(rows)
			.header(Row::new(vec![
				Span::styled("Type", Style::default().add_modifier(Modifier::BOLD)),
				Span::styled("Contents", Style::default().add_modifier(Modifier::BOLD)),
			]))
			.block(
				Block::default()
					.borders(Borders::ALL)
					.style(Style::default().fg(Color::White))
					.title("Quote Details")
					.border_type(BorderType::Plain),
			)
			.widths(&[Constraint::Percentage(33), Constraint::Percentage(66)])
	} else {
		Table::new(vec![]).block(
			Block::default()
				.borders(Borders::ALL)
				.style(Style::default().fg(Color::White))
				.title("No Quotes to List")
				.border_type(BorderType::Plain),
		)
	};
	
	let list = List::new(get_type_items()).block(quotes).highlight_style(
		Style::default()
			.bg(Color::Yellow)
			.fg(Color::Black)
			.add_modifier(Modifier::BOLD),
	);
	
	(list, quote_detail)
}

pub fn render_entry(
	current_input: &str,
) -> (List, Paragraph) {
	let block = Block::default()
		.borders(Borders::ALL)
		.style(Style::default().fg(Color::White))
		.title("Quote Type")
		.border_type(BorderType::Plain);
	
	let list = List::new(get_type_items()).block(block).highlight_style(
		Style::default()
			.bg(Color::Yellow)
			.fg(Color::Black)
			.add_modifier(Modifier::BOLD),
	);
	
	let para = Paragraph::new(vec![
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::raw(
			"Enter in your Quote, press Enter to Confirm",
		)]),
		Spans::from(vec![Span::raw("")]),
		Spans::from(vec![Span::raw(current_input)]),
		Spans::from(vec![Span::raw("")]),
	])
		.alignment(Alignment::Center)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.style(Style::default().fg(Color::White))
				.title("Quote Entry")
				.border_type(BorderType::Plain),
		);
	
	(list, para)
}

