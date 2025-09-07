use crate::app_state::AppState;
use eframe::egui;

pub fn app_content(state: &mut AppState, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show_viewport(ui, |ui, viewport| {
                ui.add(
                    egui::TextEdit::multiline(&mut state.file_content)
                        .frame(false)
                        .desired_width(f32::INFINITY)
                        .desired_rows(50)
                        .code_editor()
                );
            });
    });
}