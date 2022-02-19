mod db;
mod quote;
mod rendering;
mod util;

use crate::{
    db::{add_quote_to_db, read_db, remove_quote_by_quote},
    quote::{Quote, ALL_PERMS},
    rendering::{render_category_quotes, render_entry, render_home, render_quotes},
    util::{Event, MenuItem},
};
use crossterm::{
    event,
    event::{Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, ListItem, ListState, Paragraph, Tabs},
    Terminal,
};

//based off https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    color_eyre::install()?;

    enable_raw_mode().unwrap();

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(20);
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

    let mut types_list_state = ListState::default();
    types_list_state.select(Some(0));

    let mut category_quotes_selection_state = ListState::default();
    category_quotes_selection_state.select(Some(0));

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

            let copyright = Paragraph::new("quotes-tui 2022 - All rights reserved")
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
                    let entry_chunk = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(33), Constraint::Percentage(67)].as_ref(),
                        )
                        .split(chunks[1]);

                    let (types, entry) = render_entry(current_input.as_str());
                    rect.render_stateful_widget(types, entry_chunk[0], &mut types_list_state);
                    rect.render_widget(entry, entry_chunk[1]);
                }
                MenuItem::QuoteCategory => {
                    let q = ALL_PERMS[quotes_list_state.selected().expect("quote type selected")];
                    let db = read_db().expect("can read db");
                    let qs = db
                        .into_iter()
                        .filter(|quote| quote.1 == q)
                        .map(|quote| ListItem::new(quote.0))
                        .collect();

                    let widget = render_category_quotes(qs);
                    rect.render_stateful_widget(
                        widget,
                        chunks[1],
                        &mut category_quotes_selection_state,
                    );
                }
                _ => {}
            }
        })?;

        if let Ok(event) = rx.recv() {
            match active_menu_item {
                MenuItem::Entry => {
                    if let Event::Input(event) = event {
                        match event.code {
                            KeyCode::Esc => active_menu_item = MenuItem::Quotes,
                            KeyCode::Enter => {
                                add_quote_to_db(Quote(
                                    current_input.trim().to_string(),
                                    ALL_PERMS[types_list_state.selected().expect("type selected")],
                                ))
                                .expect("cannot add quote");
                                current_input.clear();
                            }
                            KeyCode::Backspace => {
                                if !current_input.is_empty() {
                                    current_input.remove(current_input.len() - 1);
                                }
                            }
                            KeyCode::Down => {
                                if let Some(selected) = types_list_state.selected() {
                                    let amt_types = ALL_PERMS.len();
                                    if selected >= amt_types - 1 {
                                        types_list_state.select(Some(0));
                                    } else {
                                        types_list_state.select(Some(selected + 1))
                                    }
                                }
                            }
                            KeyCode::Up => {
                                if let Some(selected) = types_list_state.selected() {
                                    let amt_types = ALL_PERMS.len();
                                    if selected > 0 {
                                        types_list_state.select(Some(selected - 1));
                                    } else {
                                        types_list_state.select(Some(amt_types - 1))
                                    }
                                }
                            }
                            KeyCode::Char(char) => {
                                current_input.push(char);
                            }
                            _ => {}
                        }
                    }
                }
                MenuItem::QuoteCategory => {
                    if let Event::Input(event) = event {
                        let amt_quotes = {
                            let q = ALL_PERMS
                                [quotes_list_state.selected().expect("quote type selected")];
                            read_db()
                                .expect("can read db")
                                .iter()
                                .filter(|quote| quote.1 == q)
                                .count()
                        };
                        match event.code {
                            KeyCode::Down => {
                                if let Some(selected) = category_quotes_selection_state.selected() {
                                    if selected >= amt_quotes - 1 {
                                        category_quotes_selection_state.select(Some(0));
                                    } else {
                                        category_quotes_selection_state.select(Some(selected + 1))
                                    }
                                }
                            }
                            KeyCode::Up => {
                                if let Some(selected) = category_quotes_selection_state.selected() {
                                    if selected > 0 {
                                        category_quotes_selection_state.select(Some(selected - 1));
                                    } else {
                                        category_quotes_selection_state.select(Some(amt_quotes - 1))
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                category_quotes_selection_state.select(Some(0));
                                active_menu_item = MenuItem::Quotes;
                            }
                            KeyCode::Enter => {
                                let quote_type_index =
                                    quotes_list_state.selected().expect("quote type selected");
                                let db = read_db().expect("can read db");
                                let qq = {
                                    let q = ALL_PERMS[quote_type_index];
                                    let quotes_all: Vec<_> = db
                                        .clone()
                                        .into_iter()
                                        .filter(|quote| quote.1 == q)
                                        .collect();
                                    quotes_all[category_quotes_selection_state
                                        .selected()
                                        .unwrap_or_default()]
                                    .clone()
                                };

                                remove_quote_by_quote(
                                    &mut category_quotes_selection_state,
                                    qq.clone(),
                                )
                                .expect("cannot remove quote");
                                current_input = qq.0;
                                active_menu_item = MenuItem::Entry;
                                types_list_state.select(Some(quote_type_index));
                            }
                            KeyCode::Char('d') => {
                                let quote_type_index =
                                    quotes_list_state.selected().expect("quote type selected");
                                let db = read_db().expect("can read db");
                                let qq = {
                                    let q = ALL_PERMS[quote_type_index];
                                    let quotes_all: Vec<_> = db
                                        .clone()
                                        .into_iter()
                                        .filter(|quote| quote.1 == q)
                                        .collect();
                                    quotes_all[category_quotes_selection_state
                                        .selected()
                                        .unwrap_or_default()]
                                    .clone()
                                };

                                remove_quote_by_quote(
                                    &mut category_quotes_selection_state,
                                    qq.clone(),
                                )
                                .expect("cannot remove quote");
                            }
                            _ => {}
                        }
                    }
                }
                _ => match event {
                    Event::Input(event) => match event.code {
                        KeyCode::Char('g') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('q') => active_menu_item = MenuItem::Quotes,
                        KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                        KeyCode::Char('e') => {
                            current_input.clear();
                            active_menu_item = MenuItem::Entry;
                        }
                        KeyCode::Char('d') => {
                            let quote_type_index =
                                quotes_list_state.selected().expect("quote type selected");
                            let db = read_db().expect("can read db");
                            let qq = {
                                let q = ALL_PERMS[quote_type_index];
                                let quotes_all: Vec<_> = db
                                    .clone()
                                    .into_iter()
                                    .filter(|quote| quote.1 == q)
                                    .collect();
                                quotes_all[category_quotes_selection_state
                                    .selected()
                                    .unwrap_or_default()]
                                .clone()
                            };

                            remove_quote_by_quote(&mut quotes_list_state, qq)
                                .expect("can remove quote");
                        }
                        KeyCode::Enter => {
                            active_menu_item = MenuItem::QuoteCategory;
                        }
                        KeyCode::Down => {
                            if let Some(selected) = quotes_list_state.selected() {
                                let amt_types = ALL_PERMS.len();
                                if amt_types != 0 {
                                    if selected >= amt_types - 1 {
                                        quotes_list_state.select(Some(0));
                                    } else {
                                        quotes_list_state.select(Some(selected + 1))
                                    }
                                }
                            }
                        }
                        KeyCode::Up => {
                            if let Some(selected) = quotes_list_state.selected() {
                                let amt_types = ALL_PERMS.len();
                                if amt_types != 0 {
                                    if selected > 0 {
                                        quotes_list_state.select(Some(selected - 1));
                                    } else {
                                        quotes_list_state.select(Some(amt_types - 1))
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
