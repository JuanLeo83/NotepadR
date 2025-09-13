use crate::app_state::{get_available_fonts, AppState, Language};
use crate::navigator::Screen;
use eframe::egui;
use rfd::FileDialog;

pub fn settings_screen(state: &mut AppState, ctx: &egui::Context, frame: &mut eframe::Frame) {
    settings_content(state, ctx, frame);
}

fn settings_content(state: &mut AppState, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show_viewport(ui, |ui, _viewport| {
                ui.vertical(|ui| {
                    ui.heading("Configuración");
                    ui.add_space(20.0);

                    // Ligh or dark theme
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Tema:");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut state.settings_state.unsaved.dark_mode, true, "Oscuro");
                            ui.radio_value(&mut state.settings_state.unsaved.dark_mode, false, "Claro");
                        });
                    });

                    ui.add_space(10.0);

                    // Font
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Fuente:");
                        egui::ComboBox::from_id_salt("Font")
                            .selected_text(&state.settings_state.unsaved.font_name)
                            .show_ui(ui, |ui| {
                                for font_name in get_available_fonts() {
                                    ui.selectable_value(
                                        &mut state.settings_state.unsaved.font_name,
                                        font_name.clone(),
                                        font_name
                                    );
                                }
                            });

                        ui.label("Tamaño de fuente:");
                        ui.add(egui::Slider::new(&mut state.settings_state.unsaved.font_size, 8.0..=24.0).text("pt"));
                    });

                    ui.add_space(10.0);

                    // Default path for files
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Ruta por defecto:");
                        ui.horizontal(|ui| {
                            let text_edit = ui.text_edit_singleline(&mut state.settings_state.unsaved.default_path);
                            if ui.button("Examinar...").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .set_title("Seleccionar carpeta")
                                    .pick_folder() {
                                    state.settings_state.unsaved.default_path = path.to_string_lossy().into_owned();
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Language
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label("Idioma:");
                        egui::ComboBox::from_id_salt("Language")
                            .selected_text(get_language(&state.settings_state.unsaved.language))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut state.settings_state.unsaved.language, Language::Spanish, get_language(&Language::Spanish));
                                ui.selectable_value(&mut state.settings_state.unsaved.language, Language::English, get_language(&Language::English));
                                ui.selectable_value(&mut state.settings_state.unsaved.language, Language::French, get_language(&Language::French));
                            });
                    });

                    ui.add_space(10.0);

                    // Confirm before close
                    ui.checkbox(&mut state.settings_state.unsaved.confirm_on_close, "Confirmar antes de cerrar si hay cambios sin guardar");

                    // Bottom buttons
                    ui.add_space(20.0);
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Guardar").clicked() {
                                save(state);
                                state.screen = Screen::Notepad;
                            }

                            if ui.button("Aplicar").clicked() {
                                save(state);
                            }

                            if ui.button("Cancelar").clicked() {
                                discard_changes(state);
                                state.screen = Screen::Notepad;
                            }
                        });
                    });
                });
            });
    });
}

fn save(state: &mut AppState) {
    state.settings_state.current = state.settings_state.unsaved.clone();

    if let Err(err) = state.save_settings_to_disk() {
        eprintln!("Error al guardar la configuración: {}", err);
    }
}

fn discard_changes(state: &mut AppState) {
    state.settings_state.unsaved = state.settings_state.current.clone()
}

fn get_language(language: &Language) -> &'static str {
    match language {
        Language::Spanish => "Español",
        Language::English => "English",
        Language::French => "Français",
    }
}