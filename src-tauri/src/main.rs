// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Builder;
use tauri::WindowEvent;
mod logger;

#[tauri::command]
fn log_frontend_error(level: String, message: String) {
    logger::process_frontend_error(&level, &message);
}

fn main() {
    logger::init().expect("logger init failed");
    log::info!("Annex POS is starting");

    let _ = Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![log_frontend_error,])
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                log::info!("Annex POS is exiting");
            }
        })
        .run(tauri::generate_context!());
}
