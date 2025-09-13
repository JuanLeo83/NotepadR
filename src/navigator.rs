use crate::app_state::AppState;
use crate::screens::notepad_screen::notepad_screen;
use crate::screens::settings_screen::settings_screen;
use eframe::egui;

pub fn navigator(state: &mut AppState, ctx: &egui::Context, frame: &mut eframe::Frame) {
    match state.screen {
        Screen::Notepad => notepad_screen(state, ctx, frame),
        Screen::Settings => settings_screen(state, ctx, frame)
    };
}

pub enum Screen {
    Notepad,
    Settings
}