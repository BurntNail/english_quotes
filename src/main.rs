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

    let mut current_input = String::new();

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
                MenuItem::Entry => {
                    let para = Paragraph::new(vec![
                        Spans::from(vec![Span::raw("")]),
                        Spans::from(vec![Span::raw("Type C or T, a code, a space and then your quote")]),
                        Spans::from(vec![Span::raw("")]),
                        Spans::from(vec![Span::raw(current_input.as_str())]),
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
                    rect.render_widget(para, chunks[1]);
                }
                _ => {}
            }
        })?;

        if let Ok(event) = rx.recv() {
            match active_menu_item {
                MenuItem::Entry => match event {
                    Event::Input(event) => match event.code {
                        KeyCode::Esc => active_menu_item = MenuItem::Home,
                        KeyCode::Enter => {
                            add_quote_to_db(Quote::from(current_input.trim()))
                                .expect("cannot add quote");
                        },
                        KeyCode::Backspace => {
                            if current_input.len() > 0 {
                                current_input.remove(current_input.len() - 1);
                            }
                        },
                        KeyCode::Char(char) => {
                            current_input.push(char);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => match event {
                    Event::Input(event) => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('c') => active_menu_item = MenuItem::Quotes,
                        KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                        KeyCode::Char('e') => {
                            current_input.clear();
                            active_menu_item = MenuItem::Entry;
                        }
                        KeyCode::Char('a') => {
                            add_random_quote().expect("cannot add rnd quote");
                        }
                        KeyCode::Char('d') => {
                            remove_quote_at_index(&mut quotes_list_state)
                                .expect("can remove quote");
                        }
                        KeyCode::Down => {
                            if let Some(selected) = quotes_list_state.selected() {
                                let amt_quotes = read_db().expect("can fetch quotes list").len();
                                if amt_quotes != 0 {
                                    if selected >= amt_quotes - 1 {
                                        quotes_list_state.select(Some(0));
                                    } else {
                                        quotes_list_state.select(Some(selected + 1))
                                    }
                                }
                            }
                        }
                        KeyCode::Up => {
                            if let Some(selected) = quotes_list_state.selected() {
                                let amt_quotes = read_db().expect("can fetch quotes list").len();
                                if amt_quotes != 0 {
                                    if selected > 0 {
                                        quotes_list_state.select(Some(selected - 1));
                                    } else {
                                        quotes_list_state.select(Some(amt_quotes - 1))
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    Event::Tick => {}
                },
            }
        }
    }

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
                format!("{}", quote.1),
                Style::default(),
            )]))
        })
        .collect();
    
    let quote_detail = if quotes_list.len() > 0 {
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
                    .title("Detail")
                    .border_type(BorderType::Plain),
            )
            .widths(&[Constraint::Percentage(33), Constraint::Percentage(66)])
    } else {
        Table::new(vec![])
    };

    let list = List::new(items).block(quotes).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    

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
        
        if selected != 0 {
            quote_list_state.select(Some(selected - 1));
        }
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
