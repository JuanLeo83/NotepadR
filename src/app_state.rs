use crate::content::app_content;
use crate::menu_topbar::app_menu_topbar;
use eframe::egui;
use std::path::PathBuf;

pub struct AppState {
    pub current_content: String,
    pub current_file_path: Option<PathBuf>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_content: String::new(),
            current_file_path: None,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            app_menu_topbar(self, ctx, frame);
            app_content(self, ctx, frame);
        });
    }
}