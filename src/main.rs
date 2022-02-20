mod db;
mod quote;
mod rendering;
mod util;

use crate::{
    db::{add_quote_to_db, read_db, remove_quote_by_quote},
    quote::{Quote, ALL_PERMS},
    rendering::{render_category_quotes, render_entry, render_home, render_quotes},
    util::{default_state, down_arrow, get_quote, up_arrow, Event, MenuItem},
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
    widgets::{Block, BorderType, Borders, ListItem, Paragraph, Tabs},
    Terminal,
};

//based off https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    color_eyre::install()?;

    enable_raw_mode()?;

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

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let menu_titles = vec!["View", "Quotes", "Entry"];
    let mut active_menu_item = MenuItem::Home;

    let mut quotes_viewer_main_state = default_state();
    let mut quote_entry_type_state = default_state();
    let mut quotes_main_viewer_category_state = default_state();

    let mut current_input = String::new();

    //region ui stuff that isn't re-allocated
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
        );

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

    let menu: Vec<Spans> = menu_titles
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

    let vertical_menu_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref());

    //endregion

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = chunks.split(size);

            rect.render_widget(copyright.clone(), chunks[2]);

            let tabs = Tabs::new(menu.clone())
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            let vertical_menu_chunk = vertical_menu_chunk.split(chunks[1]);

            rect.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Quotes => {
                    let (left, right) = render_quotes(&quotes_viewer_main_state);
                    rect.render_stateful_widget(
                        left,
                        vertical_menu_chunk[0],
                        &mut quotes_viewer_main_state,
                    );
                    rect.render_widget(right, vertical_menu_chunk[1]);
                }
                MenuItem::Entry => {
                    let (types, entry) = render_entry(current_input.as_str());
                    rect.render_stateful_widget(
                        types,
                        vertical_menu_chunk[0],
                        &mut quote_entry_type_state,
                    );
                    rect.render_widget(entry, vertical_menu_chunk[1]);
                }
                MenuItem::QuoteCategory => {
                    let q = ALL_PERMS[quotes_viewer_main_state
                        .selected()
                        .expect("quote type selected")];
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
                        &mut quotes_main_viewer_category_state,
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
                                    ALL_PERMS
                                        [quote_entry_type_state.selected().expect("type selected")],
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
                                down_arrow(&mut quote_entry_type_state, ALL_PERMS.len())
                            }
                            KeyCode::Up => up_arrow(&mut quote_entry_type_state, ALL_PERMS.len()),
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
                            let q = ALL_PERMS[quotes_viewer_main_state
                                .selected()
                                .expect("quote type selected")];
                            read_db()
                                .expect("can read db")
                                .iter()
                                .filter(|quote| quote.1 == q)
                                .count()
                        };
                        match event.code {
                            KeyCode::Down => {
                                down_arrow(&mut quotes_main_viewer_category_state, amt_quotes)
                            }
                            KeyCode::Up => {
                                up_arrow(&mut quotes_main_viewer_category_state, amt_quotes)
                            }
                            KeyCode::Esc => {
                                quotes_main_viewer_category_state.select(Some(0));
                                active_menu_item = MenuItem::Quotes;
                            }
                            KeyCode::Enter => {
                                let (quote_selected, quote_type_index) =
                                    get_quote(&mut quotes_main_viewer_category_state);

                                remove_quote_by_quote(
                                    &mut quotes_main_viewer_category_state,
                                    quote_selected.clone(),
                                )
                                .expect("cannot remove quote");
                                current_input = quote_selected.0;
                                active_menu_item = MenuItem::Entry;
                                quote_entry_type_state.select(Some(quote_type_index));
                            }
                            KeyCode::Char('d') => {
                                let (quote, ..) = get_quote(&mut quotes_main_viewer_category_state);
                                remove_quote_by_quote(
                                    &mut quotes_main_viewer_category_state,
                                    quote,
                                )
                                .expect("cannot remove quote");
                            }
                            _ => {}
                        }
                    }
                }
                MenuItem::Quotes => {
                    if let Event::Input(event) = event {
                        match event.code {
                            KeyCode::Char('g') => {
                                disable_raw_mode()?;
                                terminal.show_cursor()?;
                                break;
                            }
                            KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                            KeyCode::Char('e') => {
                                current_input.clear();
                                active_menu_item = MenuItem::Entry;
                            }
                            KeyCode::Enter => {
                                active_menu_item = MenuItem::QuoteCategory;
                            }
                            KeyCode::Down => {
                                down_arrow(&mut quotes_main_viewer_category_state, ALL_PERMS.len())
                            }
                            KeyCode::Up => {
                                up_arrow(&mut quotes_main_viewer_category_state, ALL_PERMS.len())
                            }
                            _ => {}
                        }
                    }
                }
                MenuItem::Home => match event {
                    Event::Input(event) => match event.code {
                        KeyCode::Char('g') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('q') => active_menu_item = MenuItem::Quotes,
                        _ => {}
                    },
                    Event::Tick => {}
                },
                MenuItem::Quit => {}
            }
        }
    }

    Ok(())
}
