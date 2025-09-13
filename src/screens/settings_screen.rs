use crate::app_state::AppState;
use crate::navigator::Screen;
use eframe::egui;

pub fn settings_screen(state: &mut AppState, ctx: &egui::Context, frame: &mut eframe::Frame) {
    settings_content(state, ctx, frame);
}

fn settings_content(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show_viewport(ui, |ui, _viewport| {
                ui.label("Aqui van las opciones de configuración");
                if ui.button("Close").clicked() {
                    state.screen = Screen::Notepad;
                }
            });
    });
}
