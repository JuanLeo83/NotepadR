use crate::content::app_content;
use crate::menu_topbar::app_menu_topbar;
use crate::shortcuts::shortcuts;
use eframe::egui;
use std::path::PathBuf;

pub struct AppState {
    pub current_content: String,
    pub current_file_path: Option<PathBuf>,
    pub file_content: Option<String>,
    pub show_save_modal: bool,
    pub pending_action: PendingAction,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_content: String::new(),
            current_file_path: None,
            file_content: None,
            show_save_modal: false,
            pending_action: PendingAction::None,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            app_menu_topbar(self, ctx, frame);
            app_content(self, ctx, frame);

            if ctx.input(|i| i.viewport().close_requested()) {
                if self.has_unsaved_changes() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    self.pending_action = PendingAction::CloseApp;
                    self.show_save_modal = true;
                }
            }

            shortcuts(ctx, self);
        });
    }
}

impl AppState {
    pub fn has_unsaved_changes(&self) -> bool {
        match &self.file_content {
            Some(content) => self.current_content != *content,
            None => !self.current_content.is_empty()
        }
    }
}

pub enum PendingAction {
    None,
    NewFile,
    OpenFile,
    CloseApp,
}