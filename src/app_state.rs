use crate::navigator::{navigator, Screen};
use crate::shortcuts::shortcuts;
use eframe::egui;
use std::path::PathBuf;

pub struct AppState {
    pub screen: Screen,
    pub notepad_state: NotepadState,
    pub settings_state: SettingsState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Notepad,
            notepad_state: NotepadState::default(),
            settings_state: SettingsState::default(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            navigator(self, ctx, frame);

            if ctx.input(|i| i.viewport().close_requested()) {
                if self.has_unsaved_changes() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    self.notepad_state.pending_action = PendingAction::CloseApp;
                    self.notepad_state.show_save_modal = true;
                }
            }

            shortcuts(ctx, self);
        });
    }
}

impl AppState {
    pub fn has_unsaved_changes(&self) -> bool {
        match &self.notepad_state.file_content {
            Some(content) => self.notepad_state.current_content != *content,
            None => !self.notepad_state.current_content.is_empty()
        }
    }
}

pub enum PendingAction {
    None,
    NewFile,
    OpenFile,
    CloseApp,
}

pub struct NotepadState {
    pub current_content: String,
    pub current_file_path: Option<PathBuf>,
    pub file_content: Option<String>,
    pub show_save_modal: bool,
    pub pending_action: PendingAction,
}

impl Default for NotepadState {
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

pub struct SettingsState {
    pub current: Settings,
    pub unsaved: Settings,
}

#[derive(Clone)]
pub struct Settings {
    pub dark_mode: bool,
    pub font_name: String,
    pub font_size: f32,
    pub default_path: String,
    pub language: Language,
    pub confirm_on_close: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            current: Settings::default(),
            unsaved: Settings::default()
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            font_name: "Arial".to_string(),
            font_size: 12.0,
            default_path: "".to_string(),
            language: Language::English,
            confirm_on_close: true
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Language {
    English,
    Spanish,
    French,
}