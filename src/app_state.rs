use eframe::egui;

pub struct AppState {

}

impl Default for AppState {
    fn default() -> Self {
        Self {

        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}