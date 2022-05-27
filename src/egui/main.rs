//TODO: https://github.com/emilk/egui/blob/master/examples/file_dialog/src/main.rs
//TODO: Option to use eframe::Storage
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod utility;

#[macro_use]
extern crate tracing;

use crate::utility::{get_chosen_types, reverse_chosen_types, vertical_category_checkbox};
use eframe::{egui, glow::Context};
use egui::panel::Side;
use english_quotes::{
    db::{add_quote_to_db, read_db, remove_quote, sort_list},
    quote::{FileType, Quote, ALL_PERMS},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let sub = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(sub).expect("Unable to set tracing sub");

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "English Quotes",
        options,
        Box::new(|_cc| Box::new(EnglishQuotesApp::default())),
    );
}

#[derive(Clone, Debug, PartialEq)]
pub enum CurrentAppState {
    QuoteCategories,
    QuoteEntry { current_text: String },
    Search { current_search_term: String },
}

struct EnglishQuotesApp {
    current_state: CurrentAppState,
    current_db: Vec<Quote>,
    current_checked: Vec<bool>,
    quote_settings: Option<Quote>,
}

impl Default for EnglishQuotesApp {
    fn default() -> Self {
        Self {
            current_state: CurrentAppState::QuoteCategories,
            current_db: read_db().unwrap_or_else(|error| {
                warn!("Unable to read database for EQ App: {error:?}");
                vec![]
            }),
            current_checked: vec![false; ALL_PERMS.len()],
            quote_settings: None,
        }
    }
}

impl eframe::App for EnglishQuotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(Side::Left, "tab_menu").show(ctx, |ui| {
            ui.heading("Menus");

            if ui.button("All Quotes").clicked() {
                self.current_state = CurrentAppState::QuoteCategories;
            }
            if ui.button("Quote Entry").clicked() {
                self.current_state = CurrentAppState::QuoteEntry {
                    current_text: String::new(),
                };
            }
            if ui.button("Search Quotes").clicked() {
                self.current_state = CurrentAppState::Search {
                    current_search_term: String::new(),
                };
            }
        });

        {
            let mut new_qs = false;
            if let Some(quote) = &self.quote_settings {
                egui::Window::new("Quote Settings")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.heading(&quote.0);
                        if ui.button("Delete Quote").clicked() {
                            remove_quote(quote, Some(&mut self.current_db))
                                .unwrap_or_else(|err| warn!("Unable to remove quote: {err}"));
                            new_qs = true;
                        }
                        if ui.button("Edit Quote").clicked() {
                            remove_quote(quote, Some(&mut self.current_db))
                                .unwrap_or_else(|err| warn!("Unable to remove quote: {err}"));

                            let quote = quote.clone();

                            self.current_state = CurrentAppState::QuoteEntry {
                                current_text: quote.0,
                            };
                            self.current_checked = reverse_chosen_types(quote.1);

                            new_qs = true;
                        }
                        if ui.button("Cancel").clicked() {
                            new_qs = true;
                        }
                    });
            }

            if new_qs {
                self.quote_settings = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match &mut self.current_state {
                CurrentAppState::QuoteCategories => {
                    ui.heading("All Quotes");

                    ui.horizontal(|ui| {
                        vertical_category_checkbox(ui, &mut self.current_checked);
                        ui.vertical(|ui| {
                            let chosen_types: Vec<String> =
                                get_chosen_types(self.current_checked.clone());
                            let chosen_quotes =
                                self.current_db.clone().into_iter().filter(|quote| {
                                    let mut works = false;

                                    for t in &chosen_types {
                                        if quote.1.contains(t) {
                                            works = true;
                                            break;
                                        }
                                    }

                                    works
                                });

                            for quote in chosen_quotes {
                                let Quote(txt, cats) = quote.clone();
                                if ui.small_button(format!("{cats:?} | {txt}")).clicked() {
                                    self.quote_settings = Some(quote);
                                }
                            }
                        });
                    });
                }
                CurrentAppState::QuoteEntry { current_text } => {
                    ui.heading("Quote Entry");

                    ui.horizontal(|ui| {
                        vertical_category_checkbox(ui, &mut self.current_checked);
                        ui.vertical(|ui| {
                            ui.text_edit_singleline(current_text);

                            if ui.button("Submit!").clicked() {
                                let new_text = current_text.clone().trim().to_string();
                                let chosen_ts = get_chosen_types(self.current_checked.clone());
                                let new_quote = Quote(new_text, chosen_ts);

                                add_quote_to_db(new_quote, Some(&mut self.current_db))
                                    .unwrap_or_else(|err| {
                                        warn!("Unable to add quote: {err}");
                                        vec![]
                                    });

                                current_text.clear();
                                sort_list(Some(&mut self.current_db))
                                    .unwrap_or_else(|err| warn!("Unable to remove quote: {err}"));
                            }
                        });
                    });
                }
                CurrentAppState::Search {
                    current_search_term,
                } => {
                    ui.heading("Search Mode");
                    ui.horizontal(|ui| {
                        ui.label("Search Input: ");
                        ui.text_edit_singleline(current_search_term);
                    });

                    for quote in self
                        .current_db
                        .clone()
                        .into_iter()
                        .filter(|qu| qu.0.contains(current_search_term.as_str()))
                    {
                        let Quote(txt, cats) = quote.clone();
                        if ui.small_button(format!("{cats:?} | {txt}")).clicked() {
                            self.quote_settings = Some(quote);
                        }
                    }
                }
            })
        });
    }

    fn on_exit(&mut self, _gl: &Context) {
        sort_list(Some(&mut self.current_db))
            .unwrap_or_else(|err| warn!("Unable to remove quote: {err}"));

        match &serde_json::to_vec(&self.current_db) {
            Ok(v) => {
                std::fs::write(FileType::Database.get_location(), v).unwrap_or_else(|err| {
                    warn!("Unable to save db.json: {err}");
                });
            }
            Err(e) => {
                warn!("Unable to serialise database: {e:?}");
            }
        }
    }
}
