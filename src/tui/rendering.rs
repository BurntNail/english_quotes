use crate::multiple_state::{MultipleList, MultipleListItem};
use english_quotes::{
    db::read_db,
    quote::{Quote, ALL_PERMS},
    utils::either::Either,
};
use std::borrow::Cow;
use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
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
		Either::l("Press 'q' to access the Quotes, 'e' to enter a new Quote, 'f' to enter find mode, 'h' to get back home, and 'g' to exit."),
        Either::l("Once in entry mode, use arrow keys to highlight and Tab to select."),
        Either::l(""),
        Either::l("In Quote mode, use arrow keys to browse categories, tab to see just one category, and enter to edit the selected quote."),
        Either::l("In Find mode, enter text to search, and hit enter to edit the selected quote.")
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

    let quote_detail = if quotes_list.is_empty() {
        Table::new(vec![]).block(default_block().title("No Quotes to List"))
    } else {
        let selected_type = ALL_PERMS[quotes_list_state
            .selected()
            .expect("there is always a selected type in the types list")]
        .to_string();

        let rows: Vec<_> = quotes_list
            .into_iter()
            .filter(|quote| quote.1.contains(&selected_type))
            .map(|quote| {
                Row::new(vec![
                    Span::raw(format!("{:?}", quote.1)),
                    Span::raw(quote.0),
                ])
            })
            .collect();

        Table::new(rows)
            .header(Row::new(vec![
                Span::styled("Type", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("Contents", Style::default().add_modifier(Modifier::BOLD)),
            ]))
            .block(default_block().title("Quote Details"))
            .widths(&[Constraint::Percentage(33), Constraint::Percentage(66)])
    };

    let items: Vec<ListItem> = ALL_PERMS
        .iter()
        .map(|quote| ListItem::new(Spans::from(vec![Span::styled(quote, Style::default())])))
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
            MultipleListItem::new(Spans::from(vec![Span::styled(quote, Style::default())]))
        })
        .collect();

    let list = MultipleList::new(items)
        .block(block)
        .highlight_style(default_style())
        .non_select_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )
        .both_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Red)
                .fg(Color::White),
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
    .block(default_block().title("Quote Entry"))
    .wrap(Wrap { trim: true });

    (list, para)
}

pub fn render_finder(current_input: &str) -> (Paragraph, List, Vec<Quote>) {
    let db = read_db().unwrap_or_default();
    let db_len = db.len();
    let items: Vec<Quote> = db
        .into_iter()
        .filter(|quote| {
            quote
                .0
                .to_lowercase()
                .contains(&current_input.to_lowercase())
        })
        .collect();
    let items_len = items.len();

    let list = List::new(
        items
            .clone()
            .into_iter()
            .map(|quote| ListItem::new(Span::from(format!("{} - {:?}", quote.0, quote.1))))
            .collect::<Vec<ListItem>>(),
    )
    .block(default_block().title(format!("Search Results ({}/{}):", items_len, db_len)))
    .highlight_style(default_style());

    let para = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Enter in your search terms:")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(current_input)]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(default_block().title("Search Entry: "))
    .wrap(Wrap { trim: true });

    (para, list, items)
}

pub fn coloured_span<'a>(st: impl Into<Cow<'a, str>>, fg: Color) -> Span<'a> {
    Span::styled(st, Style::default().fg(fg))
}

pub fn default_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain)
}

pub fn para_from_strings<'a>(texts: Vec<Either<Cow<'a, str>, Span<'a>>>) -> Paragraph<'a> {
    let mut para_body = vec![];
    for txt in texts {
        let span = match txt {
            Either::Left(st) => Span::raw(st),
            Either::Right(sp) => sp,
        };
        para_body.push(Spans::from(span));
    }
    Paragraph::new(para_body).wrap(Wrap { trim: true })
}

pub fn default_style() -> Style {
    Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Yellow)
        .fg(Color::Black)
}
