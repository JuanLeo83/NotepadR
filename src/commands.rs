use crate::app_state::{AppState, PendingAction};
use log::{error, info};
use rfd::FileDialog;
use std::fs;

pub fn new_file(state: &mut AppState) {
    state.current_content = String::new();
    state.current_file_path = None;
    state.file_content = None;
    state.pending_action = PendingAction::None;
}

pub fn open_file(state: &mut AppState) {
    if let Some(path) = FileDialog::new()
        .set_title("Abrir archivo")
        .add_filter("Archivos de texto", &["txt"])
        .add_filter("Todos los archivos", &["*"])
        .pick_file()
    {
        match fs::read_to_string(&path) {
            Ok(content) => {
                state.current_content = content;
                state.current_file_path = Some(path);
                state.file_content = Some(state.current_content.clone());
                state.pending_action = PendingAction::None;
            }
            Err(e) => error!("Error al abrir el archivo: {}", e),
        }
    }
}

pub fn save(state: &mut AppState) {
    if let Some(path) = &state.current_file_path {
        match fs::write(path, &state.current_content) {
            Ok(_) => info!("Archivo guardado en {:?}", path),
            Err(e) => error!("Error al guardar el archivo: {}", e),
        }
    } else {
        if let Some(path) = FileDialog::new()
            .set_title("Guardar archivo")
            .add_filter("Archivos de texto", &["txt"])
            .save_file()
        {
            match fs::write(&path, &state.current_content) {
                Ok(_) => {
                    info!("Archivo guardado en {:?}", path);
                    state.current_file_path = Some(path);
                    state.file_content = Some(state.current_content.clone());
                    state.pending_action = PendingAction::None;
                }
                Err(e) => error!("Error al guardar el archivo: {}", e),
            }
        }
    }
}
