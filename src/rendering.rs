use crate::{
    db::read_db,
    multiple_state::{MultipleList, MultipleListItem},
    quote::ALL_PERMS,
    utils::{
        either::Either,
        render::{coloured_span, default_block, default_style, para_from_strings},
    },
};
use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, ListState, Paragraph, Row, Table},
};

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = para_from_strings(vec![
		Either::l(""),
		Either::l("Welcome"),
		Either::l(""),
		Either::l("to"),
		Either::l(""),
		Either::r(coloured_span("quotes-TUI", Color::LightBlue)),
		Either::l(""),
		Either::l("Press 'q' to access the Quotes, 'e' to enter a new Quote, 'h' to get back home, and 'g' to exit.")
	])
		.alignment(Alignment::Center)
		.block(
			default_block().title("Home")
		);
    home
}

pub fn render_quotes<'a>(quotes_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let quotes = default_block().title("Quotes");

    let quotes_list = read_db().expect("can fetch quotes list");

    let quote_detail = if !quotes_list.is_empty() {
        let selected_type = ALL_PERMS[quotes_list_state
            .selected()
            .expect("there is always a selected type in the types list")];
        let rows: Vec<_> = quotes_list
            .into_iter()
            .filter(|quote| quote.1 == selected_type)
            .map(|quote| Row::new(vec![Span::raw(format!("{}", quote.1)), Span::raw(quote.0)]))
            .collect();

        Table::new(rows)
            .header(Row::new(vec![
                Span::styled("Type", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("Contents", Style::default().add_modifier(Modifier::BOLD)),
            ]))
            .block(default_block().title("Quote Details"))
            .widths(&[Constraint::Percentage(33), Constraint::Percentage(66)])
    } else {
        Table::new(vec![]).block(default_block().title("No Quotes to List"))
    };

    let items: Vec<ListItem> = ALL_PERMS
        .iter()
        .map(|quote| {
            ListItem::new(Spans::from(vec![Span::styled(
                format!("{}", quote),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items)
        .block(quotes)
        .highlight_style(default_style());

    (list, quote_detail)
}

pub fn render_entry(current_input: &str) -> (MultipleList, Paragraph) {
    let block = default_block().title("Quote Type");

    let items: Vec<MultipleListItem> = ALL_PERMS
        .iter()
        .map(|quote| {
            MultipleListItem::new(Spans::from(vec![Span::styled(
                format!("{}", quote),
                Style::default(),
            )]))
        })
        .collect();

    let list = MultipleList::new(items)
        .block(block)
        .highlight_style(default_style())
        .non_select_style(default_style().bg(Color::Cyan))
        .both_style(default_style().bg(Color::Green));

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
    .block(default_block().title("Quote Entry"));

    (list, para)
}
