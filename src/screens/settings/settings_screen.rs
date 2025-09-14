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
                    ui.heading(state.text("settings.title"));
                    ui.add_space(20.0);

                    // Ligh or dark theme
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(state.text("settings.theme"));
                        ui.horizontal(|ui| {
                            let dark_text = state.text("settings.theme.dark");
                            ui.radio_value(
                                &mut state.settings_state.unsaved.dark_mode,
                                true,
                                dark_text,
                            );
                            let light_text = state.text("settings.theme.light");
                            ui.radio_value(
                                &mut state.settings_state.unsaved.dark_mode,
                                false,
                                light_text,
                            );
                        });
                    });

                    ui.add_space(10.0);

                    // Font
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(state.text("settings.font"));
                        egui::ComboBox::from_id_salt("Font")
                            .selected_text(&state.settings_state.unsaved.font_name)
                            .show_ui(ui, |ui| {
                                for font_name in get_available_fonts() {
                                    // Mostrar etiqueta localizada para la opción 'Default'
                                    let label = if font_name == "Default" {
                                        state.text("settings.font.default")
                                    } else {
                                        font_name.clone()
                                    };

                                    ui.selectable_value(
                                        &mut state.settings_state.unsaved.font_name,
                                        font_name.clone(),
                                        label,
                                    );
                                }
                            });

                        ui.label(state.text("settings.font.size"));
                        let unit_text = state.text("settings.font.size.unit");
                        ui.add(
                            egui::Slider::new(
                                &mut state.settings_state.unsaved.font_size,
                                8.0..=24.0,
                            )
                            .text(unit_text),
                        );
                    });

                    ui.add_space(10.0);

                    // Default path for files
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(state.text("settings.default.path"));
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(
                                &mut state.settings_state.unsaved.default_path,
                            );
                            if ui.button(state.text("settings.default.path.button")).clicked() {
                                if let Some(path) = FileDialog::new()
                                    .set_title(state.text("settings.default.path.select.folder"))
                                    .pick_folder()
                                {
                                    state.settings_state.unsaved.default_path =
                                        path.to_string_lossy().into_owned();
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Language
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(state.text("settings.language"));
                        egui::ComboBox::from_id_salt("Language")
                            .selected_text(get_language(state, &state.settings_state.unsaved.language))
                            .show_ui(ui, |ui| {
                                let spanish_text = get_language(state, &Language::Spanish);
                                ui.selectable_value(
                                    &mut state.settings_state.unsaved.language,
                                    Language::Spanish,
                                    spanish_text,
                                );
                                    let english_text = get_language(state, &Language::English);
                                ui.selectable_value(
                                    &mut state.settings_state.unsaved.language,
                                    Language::English,
                                    english_text,
                                );
                                let french_text = get_language(state, &Language::French);
                                ui.selectable_value(
                                    &mut state.settings_state.unsaved.language,
                                    Language::French,
                                    french_text,
                                );
                            });
                    });

                    ui.add_space(10.0);

                    // Confirm before close
                    let checkbox_text = state.text("settings.close.confirmation");
                    ui.checkbox(
                        &mut state.settings_state.unsaved.confirm_on_close,
                        checkbox_text,
                    );

                    // Bottom buttons
                    ui.add_space(20.0);
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button(state.text("settings.button.save")).clicked() {
                                save(state);
                                state.screen = Screen::Notepad;
                            }

                            if ui.button(state.text("settings.button.apply")).clicked() {
                                save(state);
                            }

                            if ui.button(state.text("settings.button.cancel")).clicked() {
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
        eprintln!("ERRO: saving configuration -> {}", err);
    }

    if let Err(e) = state.load_language_strings() {
        println!("ERROR: loading translations -> {}", e);
    }
}

fn discard_changes(state: &mut AppState) {
    state.settings_state.unsaved = state.settings_state.current.clone()
}

fn get_language(state: &AppState, language: &Language) -> String {
    match language {
        Language::Spanish => state.text("settings.language.spanish"),
        Language::English => state.text("settings.language.english"),
        Language::French => state.text("settings.language.french"),
    }
}
