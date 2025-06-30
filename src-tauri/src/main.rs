// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod domain;
mod error;
mod infrastructure;
mod interface;
mod logger;
mod services;
mod test_support;

use crate::domain::repos::OperatorRepoTrait;
use crate::interface::controllers::operator_controller::OperatorController;
use infrastructure::db::create_connection;
use infrastructure::repos::SqliteOperatorRepo;
use std::sync::Arc;
use tauri::{Builder, WindowEvent};

fn main() {
    logger::init().expect("logger init failed");
    log::info!("Annex POS is starting");

    let conn = Arc::new(create_connection("annex_data.sqlite").expect("failed to open SQLite DB"));
    let op_repo: Arc<dyn OperatorRepoTrait> = Arc::new(SqliteOperatorRepo::new(Arc::clone(&conn)));

    // build controller once and share it
    let op_ctrl = Arc::new(OperatorController::new(Arc::clone(&op_repo)));

    let _ = Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(op_ctrl)
        .invoke_handler(tauri::generate_handler![
            logger::process_frontend_error,
            commands::list_operators,
            commands::get_operator,
            commands::create_operator,
            commands::update_operator,
        ])
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                log::info!("Annex POS is exiting");
            }
        })
        .manage(op_repo) // make repos available to all commands
        .run(tauri::generate_context!());
}
