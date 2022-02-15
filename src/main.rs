mod chars;
mod quote;
mod theme;

use crate::quote::{Quote, ALL_PERMS};
use chars::Character;
use crossterm::{
    event,
    event::{Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::Rng;
use std::fs::read_to_string;
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};
use theme::Theme;
use thiserror::*;
use tui::widgets::{List, ListItem, ListState, Row, Table};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};

//https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/

const DB_PATH: &str = "./db.json";

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
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::Quotes => 1,
            MenuItem::Entry => 2,
            MenuItem::Quit => 3,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    color_eyre::install().unwrap();

    enable_raw_mode().unwrap();

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    std::thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() > tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["View", "Quotes", "Entry", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    let mut quotes_list_state = ListState::default();
    quotes_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new("quotes-cli 2022 - All rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(copyright, chunks[2]);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Quotes => {
                    let quotes_chunk = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);

                    let (left, right) = render_quotes(&quotes_list_state);
                    rect.render_stateful_widget(left, quotes_chunk[0], &mut quotes_list_state);
                    rect.render_widget(right, quotes_chunk[1]);
                }
                _ => {}
            }
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('c') => active_menu_item = MenuItem::Quotes,
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('a') => {
                    add_random_quote().expect("cannot add rnd quote");
                }
                KeyCode::Char('d') => {
                    remove_quote_at_index(&mut quotes_list_state).expect("can remove quote");
                }
                KeyCode::Down => {
                    if let Some(selected) = quotes_list_state.selected() {
                        let amt_quotes = read_db().expect("can fetch quotes list").len();
                        if selected >= amt_quotes - 1 {
                            quotes_list_state.select(Some(0));
                        } else {
                            quotes_list_state.select(Some(selected + 1))
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = quotes_list_state.selected() {
                        let amt_quotes = read_db().expect("can fetch quotes list").len();
                        if selected > 0 {
                            quotes_list_state.select(Some(selected - 1));
                        } else {
                            quotes_list_state.select(Some(amt_quotes - 1))
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    // let mut input = String::new();

    // let mut chars: HashMap<Character, Vec<String>> = Default::default();
    // let mut themes: HashMap<Theme, Vec<String>> = Default::default();

    // println!("Enter a type, code and quote: ");

    // loop {
    //     std::io::stdin().read_line(&mut input)?;

    //     let input_read = input.trim();

    //     if input_read == "exit" {
    //         break;
    //     }

    //     let space_pos = input_read.chars().position(|c| c == ' ').unwrap();
    //     let is_char = &input_read[..1] == "C";
    //     let code = &input_read[1..space_pos];
    //     let quote = &input_read[space_pos+1..];

    //     let list = if is_char {
    //         chars.entry(code.try_into().unwrap_or_default()).or_default()
    //     } else {
    //         themes.entry(code.try_into().unwrap_or_default()).or_default()
    //     };
    //     list.push(quote.to_string());

    //     input.clear();
    // }

    // print_hashmap("chars.md", chars)?;
    // print_hashmap("themes.md", themes)?;

    Ok(())
}

fn render_home<'a>() -> Paragraph<'a> {
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

fn read_db() -> Result<Vec<Quote>, Error> {
    let db_content = read_to_string(DB_PATH)?;
    let parsed: Vec<Quote> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

fn render_quotes<'a>(quotes_list_state: &ListState) -> (List<'a>, Table<'a>) {
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
                quote.0.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_quote = quotes_list
        .get(
            quotes_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists");

    let list = List::new(items).block(quotes).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    // let quote_detail = Table::new(vec![Row::new(vec![
    //     Cell::from(Span::raw(format!("{}", selected_quote.1))),
    //     Cell::from(Span::raw(selected_quote.0.clone()))
    // ])])
    //     .header(Row::new(vec![
    //         Cell::from(Span::styled("Type", Style::default().add_modifier(Modifier::BOLD))),
    //         Cell::from(Span::styled("Contents", Style::default().add_modifier(Modifier::BOLD)))
    //     ]))

    let quotes_cnts = vec![
        Span::raw(format!("{}", selected_quote.1)),
        Span::raw(selected_quote.0.clone()),
    ];

    let quote_detail = Table::new(vec![Row::new(quotes_cnts)])
        .header(Row::new(vec![
            Span::styled("Type", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled("Contents", Style::default().add_modifier(Modifier::BOLD)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(33), Constraint::Percentage(66)]);

    (list, quote_detail)
}

fn add_quote_to_db(q: Quote) -> Result<Vec<Quote>, Error> {
    let db_content = read_to_string(DB_PATH)?;
    let mut parsed: Vec<Quote> = serde_json::from_str(&db_content)?;

    parsed.push(q);
    std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
    Ok(parsed)
}

fn remove_quote_at_index(quote_list_state: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = quote_list_state.selected() {
        let db_contents = read_to_string(DB_PATH)?;
        let mut parsed: Vec<Quote> = serde_json::from_str(&db_contents)?;
        parsed.remove(selected);
        std::fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
        quote_list_state.select(Some(selected - 1));
    }

    Ok(())
}

fn add_random_quote() -> Result<Vec<Quote>, Error> {
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

// fn print_hashmap(file_name: &str, map: HashMap<impl Display, Vec<String>>) -> IOResult<()> {
//     let path = Path::new(file_name);

//     let mut file = if File::open(path).is_ok() {
//         OpenOptions::new().append(true).open(path)
//     } else {
//         File::create(path)
//     }?;

//     for (title, vec) in map {
//         writeln!(file, "# {}", title)?;
//         for st in vec {
//             writeln!(file, " - *{}*", st)?;
//         }
//     }

//     Ok(())
// }
