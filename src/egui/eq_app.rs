//TODO: stop cloning so much

use crate::utility::{
    display_quotes_list, get_chosen_types, reverse_chosen_types, vertical_category_checkbox,
};
use eframe::glow::Context;
use egui::panel::Side;
use english_quotes::{
    db::{add_quote_to_db, read_db, remove_quote, sort_list},
    quote::{FileType, Quote, ALL_PERMS},
    utils::exports::export,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CurrentAppState {
    QuoteCategories,
    QuoteEntry {
        current_text: String,
    },
    Search {
        current_search_term: String,
        is_inverted: bool,
    },
}

pub struct EnglishQuotesApp {
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
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(Side::Left, "tab_menu").show(ctx, |ui| {
            ui.heading("Menus");

            if ui.button("All Quotes").clicked() {
                self.current_state = CurrentAppState::QuoteCategories;
            }
            if ui.button("Quote Entry").clicked() {
                self.current_state = CurrentAppState::QuoteEntry {
                    current_text: String::default(),
                };
            }
            if ui.button("Search Quotes").clicked() {
                self.current_state = CurrentAppState::Search {
                    current_search_term: String::default(),
                    is_inverted: false,
                };
            }
            if ui.button("Export").clicked() {
                export().unwrap_or_else(|err| warn!("Unable to export: {err}"));
            }
        });

        {
            let mut new_qs = false;
            if let Some(quote) = &self.quote_settings {
                egui::Window::new("Quote Settings")
                    .collapsible(false)
                    .resizable(true)
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

        egui::CentralPanel::default().show(ctx, |ui| match &mut self.current_state {
            CurrentAppState::QuoteCategories => {
                ui.heading("All Quotes");

                ui.horizontal(|ui| {
                    vertical_category_checkbox(ui, &mut self.current_checked);

                    egui::ScrollArea::vertical().show(ui, |ui| {
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

                            display_quotes_list(
                                chosen_quotes,
                                ui,
                                Some(|quote| self.quote_settings = Some(quote)),
                            );
                        })
                    });
                });
            }
            CurrentAppState::QuoteEntry { current_text } => {
                ui.heading("Quote Entry");

                ui.horizontal(|ui| {
                    vertical_category_checkbox(ui, &mut self.current_checked);
                    ui.vertical(|ui| {
                        ui.text_edit_singleline(current_text);

                        let chosen_ts = get_chosen_types(self.current_checked.clone());

                        if ui.button("Submit!").clicked() {
                            let new_text = current_text.clone().trim().to_string();
                            let new_quote = Quote(new_text, chosen_ts.clone());

                            add_quote_to_db(new_quote, Some(&mut self.current_db)).unwrap_or_else(
                                |err| {
                                    warn!("Unable to add quote: {err}");
                                    vec![]
                                },
                            );

                            current_text.clear();
                            sort_list(Some(&mut self.current_db))
                                .unwrap_or_else(|err| warn!("Unable to remove quote: {err}"));
                        }

                        if !chosen_ts.is_empty() {
                            ui.separator();

                            let db = self.current_db.clone();
                            let db_len = db.len();

                            //TODO: Make this into a function 
                            let mut chosen_len = 0;
                            let chosen_quotes =
                                db.into_iter().filter(|quote| {
                                    for t in &chosen_ts {
                                        if !quote.1.contains(t) {
                                            break false;
                                        }
                                    }
                                    chosen_len += 1;
                                    true
                                });

                            ui.heading(format!("Existing Quotes ({db_len}/{chosen_len}): "));

                            for quote in chosen_quotes {
                                ui.label(format!(" - {:?} | {}", quote.1, quote.0));
                            }
                        }
                    });
                });
            }
            CurrentAppState::Search {
                current_search_term,
                is_inverted,
            } => {
                let mut scroll = None;
                ui.heading("Search");

                ui.horizontal(|ui| {
                    ui.label("Search Input: ");
                    if ui.text_edit_singleline(current_search_term).changed() {
                        scroll = Some(());
                    }
                    ui.checkbox(is_inverted, "Invert");
                });

                let (search_results, total_no, search_no) = {
                    let full_list_clone = self.current_db.clone();
                    let total_no = full_list_clone.len();

                    let search_results = full_list_clone.into_iter().filter(|qu| {
                        let r = qu.0.contains(current_search_term.as_str());
                        if *is_inverted {
                            !r
                        } else {
                            r
                        }
                    });
                    let search_no = search_results.clone().count();

                    (search_results, total_no, search_no)
                };

                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                        let r = ui.separator().rect;
                        ui.heading(format!("Search Results: {search_no}/{total_no}"));
                        display_quotes_list(
                            search_results,
                            ui,
                            Some(|quote| self.quote_settings = Some(quote)),
                        );

                        if std::mem::take(&mut scroll).is_some() {
                            ui.scroll_to_rect(r, None);
                            //TODO: need to have a better solution than a separator
                        }
                    });
            }
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
