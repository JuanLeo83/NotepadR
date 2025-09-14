use crate::app_state::{AppState, PendingAction};
use crate::navigator::Screen;
use crate::screens::notepad::commands::{new_file, open_file, save};
use eframe::egui;

pub fn app_menu_topbar(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menu_topbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button(state.text("notepad.menu.file"), |ui| {
                if ui.button(state.text("notepad.menu.file.new")).clicked() {
                    on_new_button_clicked(state);
                }

                if ui.button(state.text("notepad.menu.file.open")).clicked() {
                    on_open_button_clicked(state);
                }

                if ui.button(state.text("notepad.menu.file.save")).clicked() {
                    on_save_button_clicked(state);
                }
            });

            if ui.button(state.text("notepad.menu.settings")).clicked() {
                state.screen = Screen::Settings;
            }
        });
    });
}

pub fn on_new_button_clicked(state: &mut AppState) {
    if state.has_unsaved_changes() {
        state.notepad_state.show_save_modal = true;
        state.notepad_state.pending_action = PendingAction::NewFile;
    } else {
        new_file(state);
    }
}

pub fn on_open_button_clicked(state: &mut AppState) {
    if state.has_unsaved_changes() {
        state.notepad_state.show_save_modal = true;
        state.notepad_state.pending_action = PendingAction::OpenFile;
    } else {
        open_file(state);
    }
}

pub fn on_save_button_clicked(state: &mut AppState) {
    save(state);
}
