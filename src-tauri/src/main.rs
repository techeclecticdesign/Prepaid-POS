// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod common;
mod domain;
mod infrastructure;
mod interface;
mod services;
mod test_support;

use crate::domain::repos::OperatorRepoTrait;
use crate::interface::controllers::operator_controller::OperatorController;
use infrastructure::db::create_connection;
use infrastructure::repos::SqliteOperatorRepo;
use std::sync::Arc;
use std::sync::RwLock;
use tauri::{Builder, WindowEvent};

fn main() {
    common::logger::init().expect("logger init failed");
    log::info!("Annex POS is starting");

    dotenvy::dotenv().ok();

    let conn = Arc::new(create_connection("annex_data.sqlite").unwrap_or_else(|e| {
        log::error!("DB init error: {}", e);
        std::process::exit(1);
    }));
    let op_repo: Arc<dyn OperatorRepoTrait> = Arc::new(SqliteOperatorRepo::new(Arc::clone(&conn)));

    // build controller once and share it
    let op_ctrl = Arc::new(OperatorController::new(Arc::clone(&op_repo)));

    // filter spammy tao / winit event loop spam in console
    std::env::set_var(
        "RUST_LOG",
        "info,tao::platform_impl::platform::event_loop::runner=error",
    );

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(op_ctrl)
        .manage(RwLock::new(common::auth::AuthState::default()))
        .invoke_handler(tauri::generate_handler![
            common::logger::process_frontend_error,
            commands::auth::staff_login,
            commands::auth::staff_logout,
            commands::auth::check_login_status,
            commands::auth::update_activity,
            commands::crud::operator::list_operators,
            commands::crud::operator::get_operator,
            commands::crud::operator::create_operator,
            commands::crud::operator::update_operator,
        ])
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                log::info!("Annex POS is exiting");
            }
        })
        .manage(op_repo) // make repos available to all commands
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("Tauri run failed: {}", e);
            std::process::exit(1);
        });
}
