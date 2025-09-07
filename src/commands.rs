use crate::app_state::AppState;
use log::{error, info};
use rfd::FileDialog;
use std::fs;

pub fn save(state: &mut AppState) {
    if let Some(path) = &state.current_file_path {
        match fs::write(path, &state.current_content) {
            Ok(_) => info!("Archivo guardado en {:?}", path),
            Err(e) => error!("Error al guardar el archivo: {}", e)
        }
    } else {
        if let Some(path) = FileDialog::new()
            .set_title("Guardar archivo")
            .add_filter("Archivos de texto", &["txt"])
            .save_file() {
            match fs::write(&path, &state.current_content) {
                Ok(_) => {
                    info!("Archivo guardado en {:?}", path);
                    state.current_file_path = Some(path);
                }
                Err(e) => error!("Error al guardar el archivo: {}", e)
            }
        }
    }
}