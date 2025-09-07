use eframe::egui;
use log::info;

pub fn app_menu_topbar(ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menu_topbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("Archivo", |ui| {
                if ui.button("Nuevo").clicked() {
                    // TODO: create new file
                    info!("New file clicked");
                }
                if ui.button("Abrir...").clicked() {
                    // TODO: open system file picker
                    info!("Open file clicked");
                }
                if ui.button("Guardar").clicked() {
                    // TODO: save current file
                    info!("Save file clicked");
                }
            });
        });
    });
}