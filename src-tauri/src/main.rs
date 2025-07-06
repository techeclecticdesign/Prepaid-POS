// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod common;
mod domain;
mod infrastructure;
mod interface;
mod test_support;

use crate::domain::repos::{
    InventoryTransactionRepoTrait, OperatorRepoTrait, PriceAdjustmentRepoTrait, ProductRepoTrait,
};
use crate::interface::controllers::{
    operator_controller::OperatorController, product_controller::ProductController,
    transaction_controller::TransactionController,
};
use infrastructure::db::create_connection;
use infrastructure::repos::{
    SqliteInventoryTransactionRepo, SqliteOperatorRepo, SqlitePriceAdjustmentRepo,
    SqliteProductRepo,
};
use std::sync::{Arc, RwLock};
use tauri::{Builder, WindowEvent};

fn main() {
    common::logger::init().expect("logger init failed");
    log::info!("Annex POS is starting");

    dotenvy::dotenv().ok();

    let conn = Arc::new(create_connection("annex_data.sqlite").unwrap_or_else(|e| {
        log::error!("DB init error: {}", e);
        std::process::exit(1);
    }));
    // Define dependency injected objects
    let op_repo: Arc<dyn OperatorRepoTrait> = Arc::new(SqliteOperatorRepo::new(Arc::clone(&conn)));
    let product_repo: Arc<dyn ProductRepoTrait> =
        Arc::new(SqliteProductRepo::new(Arc::clone(&conn)));
    let price_repo: Arc<dyn PriceAdjustmentRepoTrait> =
        Arc::new(SqlitePriceAdjustmentRepo::new(Arc::clone(&conn)));
    let inv_repo: Arc<dyn InventoryTransactionRepoTrait> =
        Arc::new(SqliteInventoryTransactionRepo::new(Arc::clone(&conn)));
    let op_ctrl = Arc::new(OperatorController::new(Arc::clone(&op_repo)));
    let product_ctrl = Arc::new(ProductController::new(
        Arc::clone(&product_repo),
        Arc::clone(&price_repo),
        Arc::clone(&conn),
    ));
    let tx_ctrl = Arc::new(TransactionController::new(Arc::clone(&inv_repo)));

    // filter spammy tao / winit event loop spam in console
    std::env::set_var(
        "RUST_LOG",
        "info,tao::platform_impl::platform::event_loop::runner=error",
    );

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        // put dependency injected objects in the Tauri DI container
        .manage(op_ctrl)
        .manage(product_ctrl)
        .manage(tx_ctrl)
        .manage(RwLock::new(common::auth::AuthState::default()))
        .manage(op_repo)
        .manage(product_repo)
        .manage(price_repo)
        .manage(inv_repo)
        .invoke_handler(tauri::generate_handler![
            common::logger::process_frontend_error,
            interface::commands::auth::check_login_status,
            interface::commands::auth::staff_login,
            interface::commands::auth::staff_logout,
            interface::commands::auth::update_activity,
            interface::commands::operator::create_operator,
            interface::commands::operator::get_operator,
            interface::commands::operator::list_operators,
            interface::commands::operator::update_operator,
            interface::commands::product::create_product,
            interface::commands::product::list_price_adjust,
            interface::commands::product::list_price_adjust_for_product,
            interface::commands::product::list_price_adjust_operator,
            interface::commands::product::list_price_adjust_today,
            interface::commands::product::list_products,
            interface::commands::product::list_products_category,
            interface::commands::product::price_adjustment,
            interface::commands::product::remove_product,
            interface::commands::product::update_item,
            interface::commands::transaction::get_transaction,
            interface::commands::transaction::inventory_adjustment,
            interface::commands::transaction::list_inv_adjust_today,
            interface::commands::transaction::list_inv_adjust_operator,
            interface::commands::transaction::list_inv_adjust,
            interface::commands::transaction::list_tx_for_customer,
            interface::commands::transaction::list_tx_for_product,
            interface::commands::transaction::sale_transaction,
            interface::commands::transaction::stock_items,
        ])
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                log::info!("Annex POS is exiting");
            }
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("Tauri run failed: {}", e);
            std::process::exit(1);
        });
}
