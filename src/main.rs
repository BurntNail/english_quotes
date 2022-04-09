#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

mod db;
mod multiple_state;
mod quote;
mod rendering;
mod utils;

use crate::{
    db::{
        add_quote_to_db, get_quote, get_quote_by_content, read_db, remove_quote_by_quote, sort_list,
    },
    multiple_state::MultipleListState,
    quote::{Quote, ALL_PERMS},
    rendering::{render_entry, render_finder, render_home, render_quotes},
    utils::{
        events::{default_state, down_arrow, up_arrow, Event},
        exports::export,
        render::{default_block, default_style},
        MenuItem,
    },
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
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal,
};

//based off https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
///But now much farther along than that project ever was

#[allow(clippy::too_many_lines)]
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

    let menu_titles = vec!["Home", "Quotes", "Entry", "Find"];
    let mut active_menu_item = MenuItem::Home;

    let mut main_category_state = default_state();
    let mut entry_category_state = MultipleListState::default();
    entry_category_state.highlight(Some(0));
    let mut quote_single_category_state = default_state();
    let mut find_quote_state = default_state();
    let mut find_quote_list = vec![];

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
        .block(default_block().title("Copyright"));

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

    let horizontal_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(66)].as_ref());

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
            let horiz_menu_chunk = horizontal_split.split(chunks[1]);

            rect.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::Quotes => {
                    let (left, right) = render_quotes(&main_category_state);
                    rect.render_stateful_widget(
                        left,
                        vertical_menu_chunk[0],
                        &mut main_category_state,
                    );
                    rect.render_widget(right, vertical_menu_chunk[1]);
                }
                MenuItem::Entry => {
                    let (types, entry) = render_entry(current_input.as_str());
                    rect.render_stateful_widget(
                        types,
                        vertical_menu_chunk[0],
                        &mut entry_category_state,
                    );
                    rect.render_widget(entry, vertical_menu_chunk[1]);
                }
                MenuItem::Find => {
                    let (entry, results, quotes_list) = render_finder(current_input.as_str());
                    rect.render_widget(entry, horiz_menu_chunk[0]);
                    rect.render_stateful_widget(
                        results,
                        horiz_menu_chunk[1],
                        &mut find_quote_state,
                    );
                    find_quote_list = quotes_list;
                }
                MenuItem::QuoteCategory => {
                    let q = ALL_PERMS[main_category_state.selected().expect("quote type selected")]
                        .to_string();
                    let db = read_db().expect("can read db");
                    let qs: Vec<_> = db
                        .into_iter()
                        .filter(|quote| quote.1.contains(&q))
                        .map(|quote| ListItem::new(quote.0))
                        .collect();

                    let widget = List::new(qs)
                        .block(default_block().title("Quote Category"))
                        .highlight_style(default_style());
                    rect.render_stateful_widget(
                        widget,
                        chunks[1],
                        &mut quote_single_category_state,
                    );
                }
            }
        })?;

        if let Ok(event) = rx.recv() {
            match active_menu_item {
                MenuItem::Entry => {
                    if let Event::Input(event) = event {
                        match event.code {
                            KeyCode::Esc => active_menu_item = MenuItem::Home,
                            KeyCode::Enter => {
                                let indices: Vec<String> = entry_category_state
                                    .selected()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .map(|index| ALL_PERMS[index].clone())
                                    .collect();

                                add_quote_to_db(Quote(current_input.trim().to_string(), indices))
                                    .expect("cannot add quote");
                                current_input.clear();
                            }
                            KeyCode::Backspace => {
                                if !current_input.is_empty() {
                                    current_input.remove(current_input.len() - 1);
                                }
                            }
                            KeyCode::Tab => {
                                if let Some(highlighted) = entry_category_state.highlighted() {
                                    entry_category_state.select(highlighted);
                                    // entry_category_state.highlight(None);
                                }
                            }
                            KeyCode::Down => {
                                if let Some(selected) = entry_category_state.highlighted() {
                                    if selected < ALL_PERMS.len() - 1 {
                                        entry_category_state.highlight(Some(selected + 1));
                                    }
                                } else {
                                    entry_category_state.highlight(Some(ALL_PERMS.len() - 1));
                                }
                            }
                            KeyCode::Up => {
                                if let Some(selected) = entry_category_state.highlighted() {
                                    if selected > 0 {
                                        entry_category_state.highlight(Some(selected - 1));
                                    }
                                } else {
                                    entry_category_state.highlight(Some(0));
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
                                [main_category_state.selected().expect("quote type selected")]
                            .to_string();
                            read_db()
                                .expect("can read db")
                                .iter()
                                .filter(|quote| quote.1.contains(&q))
                                .count()
                        };
                        match event.code {
                            KeyCode::Down => {
                                down_arrow(&mut quote_single_category_state, amt_quotes);
                            }
                            KeyCode::Up => up_arrow(&mut quote_single_category_state, amt_quotes),
                            KeyCode::Esc => {
                                quote_single_category_state.select(Some(0));
                                active_menu_item = MenuItem::Quotes;
                            }
                            KeyCode::Enter => {
                                let quote_selected = get_quote(
                                    &mut main_category_state,
                                    &mut quote_single_category_state,
                                );

                                entry_category_state.clear();
                                entry_category_state.select_multiple(&quote_selected.1);
                                remove_quote_by_quote(
                                    &mut quote_single_category_state,
                                    &quote_selected,
                                )
                                .expect("cannot remove quote");

                                current_input = quote_selected.0;
                                active_menu_item = MenuItem::Entry;
                            }
                            KeyCode::Char('d') => {
                                let quote = get_quote(
                                    &mut main_category_state,
                                    &mut quote_single_category_state,
                                );
                                remove_quote_by_quote(&mut quote_single_category_state, &quote)
                                    .expect("cannot remove quote");
                            }
                            KeyCode::Char('f') => {
                                current_input.clear();
                                active_menu_item = MenuItem::Find;
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
                            KeyCode::Char('f') => {
                                current_input.clear();
                                active_menu_item = MenuItem::Find;
                            }
                            KeyCode::Tab => {
                                active_menu_item = MenuItem::QuoteCategory;
                            }
                            KeyCode::Down => down_arrow(&mut main_category_state, ALL_PERMS.len()),
                            KeyCode::Up => up_arrow(&mut main_category_state, ALL_PERMS.len()),
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
                        KeyCode::Char('e') => {
                            current_input.clear();
                            active_menu_item = MenuItem::Entry;
                        }
                        KeyCode::Char('q') => active_menu_item = MenuItem::Quotes,
                        KeyCode::Char('f') => {
                            current_input.clear();
                            active_menu_item = MenuItem::Find;
                        }
                        KeyCode::Char('r') => export(),
                        _ => {}
                    },
                    Event::Tick => {}
                },
                MenuItem::Find => {
                    if let Event::Input(event) = event {
                        match event.code {
                            KeyCode::Esc => active_menu_item = MenuItem::Home,
                            KeyCode::Char(char) => {
                                find_quote_state.select(Some(0));
                                current_input.push(char);
                            }
                            KeyCode::Backspace => {
                                if !current_input.is_empty() {
                                    current_input.remove(current_input.len() - 1);
                                }
                            }
                            KeyCode::Up => up_arrow(&mut find_quote_state, find_quote_list.len()),
                            KeyCode::Down => {
                                down_arrow(&mut find_quote_state, find_quote_list.len())
                            }
                            KeyCode::Enter => {
                                let quote = get_quote_by_content(
                                    &find_quote_list
                                        [find_quote_state.selected().unwrap_or_default()],
                                );
                                match quote {
                                    Some(quote) => {
                                        entry_category_state.clear();
                                        entry_category_state.select_multiple(&quote.1);
                                        remove_quote_by_quote(
                                            &mut quote_single_category_state,
                                            &quote,
                                        )
                                        .expect("cannot remove quote");

                                        current_input = quote.0;
                                        active_menu_item = MenuItem::Entry;
                                    }
                                    None => active_menu_item = MenuItem::Quotes,
                                }

                                find_quote_list.clear();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    sort_list().unwrap();

    Ok(())
}
