use crate::app_state::{AppState, PendingAction};
use crate::commands::{new_file, open_file, save};
use eframe::egui;

pub fn app_menu_topbar(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menu_topbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("Archivo", |ui| {
                if ui.button("Nuevo").clicked() {
                    on_new_button_clicked(state);
                }

                if ui.button("Abrir...").clicked() {
                    on_open_button_clicked(state);
                }

                if ui.button("Guardar").clicked() {
                    on_save_button_clicked(state);
                }
            });
        });
    });
}

fn on_new_button_clicked(state: &mut AppState) {
    if state.has_unsaved_changes() {
        state.show_save_modal = true;
        state.pending_action = PendingAction::NewFile;
    } else {
        new_file(state);
    }
}

fn on_open_button_clicked(state: &mut AppState) {
    if state.has_unsaved_changes() {
        state.show_save_modal = true;
        state.pending_action = PendingAction::OpenFile;
    } else {
        open_file(state);
    }
}

fn on_save_button_clicked(state: &mut AppState) {
    save(state);
}
