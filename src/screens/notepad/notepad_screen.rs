use crate::app_state::{AppState, PendingAction};
use crate::screens::notepad::commands::{new_file, open_file, save};
use crate::screens::notepad::menu_topbar::app_menu_topbar;
use eframe::egui;

pub fn notepad_screen(state: &mut AppState, ctx: &egui::Context, frame: &mut eframe::Frame) {
    app_menu_topbar(state, ctx, frame);
    notepad_content(state, ctx, frame);
}

fn notepad_content(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show_viewport(ui, |ui, _viewport| {
                ui.push_id("main_text_editor", |ui| {
                    let text_edit_response = ui.add(
                        egui::TextEdit::multiline(&mut state.notepad_state.current_content)
                            .frame(false)
                            .desired_width(f32::INFINITY)
                            .desired_rows(50)
                            .code_editor(),
                    );

                    if state.notepad_state.show_save_modal {
                        text_edit_response.surrender_focus();
                    } else {
                        text_edit_response.request_focus();
                    }
                });
            });

        show_unsaved_changes_modal(ctx, state);
    });
}

fn show_unsaved_changes_modal(ctx: &egui::Context, state: &mut AppState) {
    if state.notepad_state.show_save_modal {
        egui::Window::new("Cambios sin guardar")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Hay cambios sin guardar. ¿Qué deseas hacer?");

                ui.horizontal(|ui| {
                    if ui.button("Descartar").clicked() {
                        state.notepad_state.show_save_modal = false;
                        execute_pending_action(ctx, state);
                    }

                    if ui.button("Cancelar").clicked() {
                        state.notepad_state.show_save_modal = false;
                        state.notepad_state.pending_action = PendingAction::None;
                    }

                    if ui.button("Guardar").clicked() {
                        state.notepad_state.show_save_modal = false;
                        save(state);
                        state.notepad_state.current_content = String::new();
                        execute_pending_action(ctx, state);
                    }
                });
            });
    }

    fn execute_pending_action(ctx: &egui::Context, state: &mut AppState) {
        match state.notepad_state.pending_action {
            PendingAction::None => {}
            PendingAction::NewFile => {
                new_file(state);
            }
            PendingAction::OpenFile => {
                open_file(state);
            }
            PendingAction::CloseApp => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        state.notepad_state.pending_action = PendingAction::None;
    }
}