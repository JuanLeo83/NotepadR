use crate::app_state::AppState;
use crate::navigator::Screen;
use crate::screens::notepad::menu_topbar::{on_new_button_clicked, on_open_button_clicked, on_save_button_clicked};
use eframe::egui;

pub fn shortcuts(ctx: &egui::Context, state: &mut AppState) {
    let modifier = get_modifier(ctx);

    match state.screen {
        Screen::Notepad => {
            if ctx.input(|i| i.key_pressed(egui::Key::N) && modifier) {
                on_new_button_clicked(state);
            }

            if ctx.input(|i| i.key_pressed(egui::Key::O) && modifier) {
                on_open_button_clicked(state);
            }

            if ctx.input(|i| i.key_pressed(egui::Key::S) && modifier) {
                on_save_button_clicked(state);
            }

            if ctx.input(|i| i.key_pressed(egui::Key::Comma) && modifier) {
                state.screen = Screen::Settings;
            }
        }
        Screen::Settings => {}
    }
}

fn get_modifier(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        if cfg!(target_os = "macos") {
            i.modifiers.mac_cmd
        } else {
            i.modifiers.ctrl
        }
    })
}