use crate::app_state::{AppState, PendingAction};
use log::{error, info};
use rfd::FileDialog;
use std::fs;

pub fn new_file(state: &mut AppState) {
    state.notepad_state.current_content = String::new();
    state.notepad_state.current_file_path = None;
    state.notepad_state.file_content = None;
    state.notepad_state.pending_action = PendingAction::None;
}

pub fn open_file(state: &mut AppState) {
    let mut dialog = FileDialog::new()
        .set_title("Abrir archivo")
        .add_filter("Archivos de texto", &["txt"])
        .add_filter("Todos los archivos", &["*"]);

    if !state.settings_state.current.default_path.is_empty() {
        dialog = dialog.set_directory(&state.settings_state.current.default_path);
    }

        if let Some(path) = dialog.pick_file()
    {
        match fs::read_to_string(&path) {
            Ok(content) => {
                state.notepad_state.current_content = content;
                state.notepad_state.current_file_path = Some(path);
                state.notepad_state.file_content =
                    Some(state.notepad_state.current_content.clone());
                state.notepad_state.pending_action = PendingAction::None;
            }
            Err(e) => error!("Error al abrir el archivo: {}", e),
        }
    }
}

pub fn save(state: &mut AppState) {
    if let Some(path) = &state.notepad_state.current_file_path {
        match fs::write(path, &state.notepad_state.current_content) {
            Ok(_) => info!("Archivo guardado en {:?}", path),
            Err(e) => error!("Error al guardar el archivo: {}", e),
        }
    } else {
        let mut dialog = FileDialog::new()
            .set_title("Guardar archivo")
            .add_filter("Archivos de texto", &["txt"]);

        if !state.settings_state.current.default_path.is_empty() {
            dialog = dialog.set_directory(&state.settings_state.current.default_path);
        }

        if let Some(path) = dialog.save_file() {
            match fs::write(&path, &state.notepad_state.current_content) {
                Ok(_) => {
                    info!("Archivo guardado en {:?}", path);
                    state.notepad_state.current_file_path = Some(path);
                    state.notepad_state.file_content =
                        Some(state.notepad_state.current_content.clone());
                    state.notepad_state.pending_action = PendingAction::None;
                }
                Err(e) => error!("Error al guardar el archivo: {}", e),
            }
        }
    }
}
