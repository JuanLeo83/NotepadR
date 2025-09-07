use crate::app_state::AppState;
use eframe::egui;

pub fn app_content(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show_viewport(ui, |ui, _viewport| {
                ui.push_id("main_text_editor", |ui| {
                    let text_edit_response = ui.add(
                        egui::TextEdit::multiline(&mut state.current_content)
                            .frame(false)
                            .desired_width(f32::INFINITY)
                            .desired_rows(50)
                            .code_editor(),
                    );

                    text_edit_response.request_focus();
                });
            });
    });
}
