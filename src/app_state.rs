use crate::content::app_content;
use crate::menu_topbar::app_menu_topbar;
use eframe::egui;

pub struct AppState {
    pub file_content: String
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            file_content: String::new()
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            app_menu_topbar(ctx, frame);
            app_content(self, ctx, frame);
        });
    }
}