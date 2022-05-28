//TODO: https://github.com/emilk/egui/blob/master/examples/file_dialog/src/main.rs
//TODO: Option to use eframe::Storage
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod eq_app;
mod utility;

#[macro_use]
extern crate tracing;

use crate::eq_app::EnglishQuotesApp;
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
