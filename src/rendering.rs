use tui::widgets::{Paragraph, Block, Borders, BorderType, ListState, List, Table, ListItem, Row};
use tui::text::{Span, Spans};
use tui::style::{Style, Color, Modifier};
use tui::layout::{Alignment, Constraint};
use crate::quote::ALL_PERMS;
use crate::db::read_db;

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
		Spans::from(vec![Span::raw("Press 'c' to access Characters, 'a' to add a random Quote and 'e' to enter a new Quote.")]),
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
	
	let quotes_list = read_db().expect("can fetch pet list");
	let items: Vec<_> = quotes_list
		.iter()
		.map(|quote| {
			ListItem::new(Spans::from(vec![Span::styled(
				format!("{}", quote.1),
				Style::default(),
			)]))
		})
		.collect();
	
	let quote_detail = if !quotes_list.is_empty() {
		let selected_quote = quotes_list
			.get(
				quotes_list_state
					.selected()
					.expect("there is always a selected quote"),
			)
			.expect("exists");
		
		Table::new(vec![Row::new(vec![
			Span::raw(format!("{}", selected_quote.1)),
			Span::raw(selected_quote.0.clone()),
		])])
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
	
	let list = List::new(items).block(quotes).highlight_style(
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
	let types: Vec<_> = ALL_PERMS
		.iter()
		.map(|quote| {
			ListItem::new(Spans::from(vec![Span::styled(
				format!("{}", quote),
				Style::default(),
			)]))
		})
		.collect();
	
	let block = Block::default()
		.borders(Borders::ALL)
		.style(Style::default().fg(Color::White))
		.title("Quote Type")
		.border_type(BorderType::Plain);
	
	let list = List::new(types).block(block).highlight_style(
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

