use crate::navigator::{navigator, Screen};
use crate::shortcuts::shortcuts;
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub struct AppState {
    pub screen: Screen,
    pub notepad_state: NotepadState,
    pub settings_state: SettingsState,
    pub strings: HashMap<String, String>,

    config_dir: String,
    config_file: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Notepad,
            notepad_state: NotepadState::default(),
            settings_state: SettingsState::default(),
            strings: HashMap::new(),
            config_dir: "NotepadR".to_string(),
            config_file: "config.json".to_string(),
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            self.apply_theme(ctx);
            self.apply_font_settings(ctx);

            navigator(self, ctx, frame);

            if ctx.input(|i| i.viewport().close_requested()) {
                if self.has_unsaved_changes() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    self.notepad_state.pending_action = PendingAction::CloseApp;
                    self.notepad_state.show_save_modal = true;
                }
            }

            shortcuts(ctx, self);
        });
    }
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let mut app = Self::default();

        if let Err(e) = app.load_settings_from_disk() {
            println!("ERROR: loading config -> {}", e);
        }

        if let Err(e) = app.load_language_strings() {
            println!("ERROR: loading translations -> {}", e);
        }

        app
    }

    pub fn has_unsaved_changes(&self) -> bool {
        match &self.notepad_state.file_content {
            Some(content) => self.notepad_state.current_content != *content,
            None => !self.notepad_state.current_content.is_empty()
        }
    }

    pub fn save_settings_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Config folder not found")?;
        let app_config_dir = config_dir.join(&self.config_dir);
        std::fs::create_dir_all(&app_config_dir)?;
        let config_path = app_config_dir.join(&self.config_file);

        let json = serde_json::to_string_pretty(&self.settings_state.current)?;
        std::fs::write(config_path, json)?;

        Ok(())
    }

    pub fn load_settings_from_disk(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Config folder not found")?;
        let config_path = config_dir.join(&self.config_dir).join(&self.config_file);

        if config_path.exists() {
            let json = std::fs::read_to_string(config_path)?;
            let settings: Settings = serde_json::from_str(&json)?;

            self.settings_state.current = settings;
            self.settings_state.unsaved = self.settings_state.current.clone();
        }

        Ok(())
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        if self.settings_state.current.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
    }

    fn apply_font_settings(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let font_size = self.settings_state.current.font_size;

        let font_family = match self.settings_state.current.font_name.as_str() {
            "Roboto" => egui::FontFamily::Name("Roboto".into()),
            "Inter" => egui::FontFamily::Name("Inter".into()),
            "Fira Code" => egui::FontFamily::Name("Fira Code".into()),
            _ => egui::FontFamily::Proportional,
        };

        for (_text_style, font_id) in style.text_styles.iter_mut() {
            font_id.size = font_size;
            font_id.family = font_family.clone();
        }

        ctx.set_style(style);
    }

    pub(crate) fn load_language_strings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let lang = match self.settings_state.current.language {
            Language::Spanish => "es",
            Language::English => "en",
            Language::French => "fr",
        };

        let path = format!("assets/strings/{}.json", lang);
        let json = std::fs::read_to_string(&path)?;
        self.strings = serde_json::from_str(&json)?;

        Ok(())
    }

    pub fn text(&self, key: &str) -> String {
        self.strings
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

pub enum PendingAction {
    None,
    NewFile,
    OpenFile,
    CloseApp,
}

pub struct NotepadState {
    pub current_content: String,
    pub current_file_path: Option<PathBuf>,
    pub file_content: Option<String>,
    pub show_save_modal: bool,
    pub pending_action: PendingAction,
}

impl Default for NotepadState {
    fn default() -> Self {
        Self {
            current_content: String::new(),
            current_file_path: None,
            file_content: None,
            show_save_modal: false,
            pending_action: PendingAction::None,
        }
    }
}

pub struct SettingsState {
    pub current: Settings,
    pub unsaved: Settings,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            current: Settings::default(),
            unsaved: Settings::default(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub dark_mode: bool,
    pub font_name: String,
    pub font_size: f32,
    pub default_path: String,
    pub language: Language,
    pub confirm_on_close: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            font_name: "Roboto".to_string(),
            font_size: 12.0,
            default_path: "".to_string(),
            language: Language::English,
            confirm_on_close: true,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    French,
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Roboto".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("../assets/fonts/Roboto-Regular.ttf"))),
    );

    fonts.font_data.insert(
        "Inter".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("../assets/fonts/Inter_18pt-Regular.ttf"))),
    );

    fonts.font_data.insert(
        "Fira Code".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("../assets/fonts/FiraCode-Regular.ttf"))),
    );

    fonts.families.insert(
        egui::FontFamily::Name("Roboto".into()),
        vec!["Roboto".to_owned()],
    );

    fonts.families.insert(
        egui::FontFamily::Name("Inter".into()),
        vec!["Inter".to_owned()],
    );

    fonts.families.insert(
        egui::FontFamily::Name("Fira Code".into()),
        vec!["Fira Code".to_owned()],
    );

    ctx.set_fonts(fonts);
}


pub fn get_available_fonts() -> Vec<String> {
    vec![
        "Default".to_string(),
        "Roboto".to_string(),
        "Inter".to_string(),
        "Fira Code".to_string(),
    ]
}
