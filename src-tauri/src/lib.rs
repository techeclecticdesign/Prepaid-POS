pub mod application;
pub mod common;
pub mod domain;
pub mod infrastructure;
pub mod interface;
pub mod test_support;

use crate::application::use_cases::legacy_migration_usecases::LegacyMigrationDeps;
use crate::application::use_cases::printer_usecases::PrinterUseCases;
use crate::domain::repos::{
    CategoryRepoTrait, ClubImportRepoTrait, ClubTransactionRepoTrait, CredentialRepoTrait,
    CustomerRepoTrait, CustomerTransactionRepoTrait, CustomerTxDetailRepoTrait,
    InventoryTransactionRepoTrait, OperatorRepoTrait, PriceAdjustmentRepoTrait, ProductRepoTrait,
    WeeklyLimitRepoTrait,
};

use crate::interface::controllers::auth_controller::AuthController;
use crate::interface::controllers::club_controller::ClubController;
use crate::interface::controllers::legacy_migration_controller::LegacyMigrationController;
use crate::interface::controllers::operator_controller::OperatorController;
use crate::interface::controllers::parse_pdf_controller::PdfParseController;
use crate::interface::controllers::pos_controller::PosController;
use crate::interface::controllers::printer_controller::PrinterController;
use crate::interface::controllers::product_controller::ProductController;
use crate::interface::controllers::transaction_controller::{
    TransactionController, TransactionControllerDeps,
};

use infrastructure::command_runner::{CommandRunner, WindowsCommandRunner};
use infrastructure::db::create_connection;
use infrastructure::pdf_parser::LopdfParser;
use infrastructure::repos::{
    SqliteCategoryRepo, SqliteClubImportRepo, SqliteClubTransactionRepo, SqliteCredentialRepo,
    SqliteCustomerRepo, SqliteCustomerTransactionRepo, SqliteCustomerTxDetailRepo,
    SqliteInventoryTransactionRepo, SqliteOperatorRepo, SqlitePriceAdjustmentRepo,
    SqliteProductRepo, SqliteWeeklyLimitRepo,
};
use std::sync::{Arc, RwLock};
use tauri::{Builder, WindowEvent};

pub fn run() {
    // Initialize logger
    common::logger::init().unwrap_or_else(|e| {
        eprintln!("Logger init failed: {e}");
        std::process::exit(1);
    });
    log::info!("Annex POS is starting");

    dotenvy::dotenv().ok();

    let conn = Arc::new(create_connection("annex_data.sqlite").unwrap_or_else(|e| {
        log::error!("DB init error: {e}");
        std::process::exit(1);
    }));
    let auth_state = Arc::new(RwLock::new(common::auth::AuthState::default()));
    // Define dependency injected objects
    let cred_repo: Arc<dyn CredentialRepoTrait> =
        Arc::new(SqliteCredentialRepo::new(Arc::clone(&conn)));
    let category_repo: Arc<dyn CategoryRepoTrait> =
        Arc::new(SqliteCategoryRepo::new(Arc::clone(&conn)));
    let op_repo: Arc<dyn OperatorRepoTrait> = Arc::new(SqliteOperatorRepo::new(Arc::clone(&conn)));
    let product_repo: Arc<dyn ProductRepoTrait> =
        Arc::new(SqliteProductRepo::new(Arc::clone(&conn)));
    let price_repo: Arc<dyn PriceAdjustmentRepoTrait> =
        Arc::new(SqlitePriceAdjustmentRepo::new(Arc::clone(&conn)));
    let inv_repo: Arc<dyn InventoryTransactionRepoTrait> =
        Arc::new(SqliteInventoryTransactionRepo::new(Arc::clone(&conn)));
    let customer_repo: Arc<dyn CustomerRepoTrait> =
        Arc::new(SqliteCustomerRepo::new(Arc::clone(&conn)));
    let club_tx_repo: Arc<dyn ClubTransactionRepoTrait> =
        Arc::new(SqliteClubTransactionRepo::new(Arc::clone(&conn)));
    let club_import_repo: Arc<dyn ClubImportRepoTrait> =
        Arc::new(SqliteClubImportRepo::new(Arc::clone(&conn)));
    let cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait> =
        Arc::new(SqliteCustomerTransactionRepo::new(Arc::clone(&conn)));
    let cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait> =
        Arc::new(SqliteCustomerTxDetailRepo::new(Arc::clone(&conn)));
    let limit_repo: Arc<dyn WeeklyLimitRepoTrait> =
        Arc::new(SqliteWeeklyLimitRepo::new(Arc::clone(&conn)));
    let runner: Arc<dyn CommandRunner> = Arc::new(WindowsCommandRunner);
    let auth_ctrl = Arc::new(AuthController::new(auth_state.clone(), cred_repo.clone()));
    let op_ctrl = Arc::new(OperatorController::new(Arc::clone(&op_repo)));
    let product_ctrl = Arc::new(ProductController::new(
        Arc::clone(&product_repo),
        Arc::clone(&price_repo),
        Arc::clone(&category_repo),
        Arc::clone(&conn),
    ));
    let tx_ctrl = Arc::new(TransactionController::new(TransactionControllerDeps {
        inv_repo: Arc::clone(&inv_repo),
        cust_tx_repo: Arc::clone(&cust_tx_repo),
        cust_tx_detail_repo: Arc::clone(&cust_tx_detail_repo),
        limit_repo: Arc::clone(&limit_repo),
        runner: Arc::clone(&runner),
        customer_repo: Arc::clone(&customer_repo),
        prod_repo: Arc::clone(&product_repo),
        conn: Arc::clone(&conn),
    }));
    let club_ctrl = Arc::new(ClubController::new(
        Arc::clone(&customer_repo),
        Arc::clone(&club_tx_repo),
    ));
    let pos_ctrl = Arc::new(PosController::new(
        Arc::clone(&product_repo),
        Arc::clone(&customer_repo),
    ));
    let legacy_ctrl = Arc::new(LegacyMigrationController::new(LegacyMigrationDeps {
        op_repo: Arc::clone(&op_repo),
        product_repo: Arc::clone(&product_repo),
        category_repo: Arc::clone(&category_repo),
        customer_repo: Arc::clone(&customer_repo),
        club_transaction_repo: Arc::clone(&club_tx_repo),
        club_imports_repo: Arc::clone(&club_import_repo),
        inv_repo: Arc::clone(&inv_repo),
        customer_transaction_repo: Arc::clone(&cust_tx_repo),
        cust_tx_detail_repo: Arc::clone(&cust_tx_detail_repo),
        sqlite_conn: Arc::clone(&conn),
    }));
    let pdf_ctrl = Arc::new(PdfParseController::new(
        Arc::new(LopdfParser),
        Arc::clone(&club_import_repo),
        Arc::clone(&club_tx_repo),
        Arc::clone(&customer_repo),
    ));

    let printer_uc = PrinterUseCases::new(
        Arc::clone(&runner),
        Arc::clone(&customer_repo),
        Arc::clone(&product_repo),
    );
    let printer_ctrl = Arc::new(PrinterController::new(printer_uc));

    // filter spammy tao / winit event loop spam in console
    std::env::set_var(
        "RUST_LOG",
        "info,tao::platform_impl::platform::event_loop::runner=error",
    );

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        // put dependency injected objects in the Tauri DI container
        .manage(op_ctrl)
        .manage(product_ctrl)
        .manage(tx_ctrl)
        .manage(club_ctrl)
        .manage(pos_ctrl)
        .manage(printer_ctrl)
        .manage(legacy_ctrl)
        .manage(pdf_ctrl)
        .manage(auth_ctrl)
        .manage(RwLock::new(common::auth::AuthState::default()))
        .manage(cred_repo)
        .manage(op_repo)
        .manage(product_repo)
        .manage(price_repo)
        .manage(inv_repo)
        .manage(club_import_repo)
        .manage(club_tx_repo)
        .manage(customer_repo)
        .manage(limit_repo)
        // add commands
        .invoke_handler(tauri::generate_handler![
            common::logger::process_frontend_error,
            interface::commands::auth::check_login_status,
            interface::commands::auth::staff_login,
            interface::commands::auth::staff_logout,
            interface::commands::auth::update_activity,
            interface::commands::auth::change_password,
            interface::commands::auth::password_required,
            interface::commands::auth::delete_password,
            interface::commands::operator::create_operator,
            interface::commands::operator::list_operators,
            interface::commands::operator::update_operator,
            interface::commands::product::create_product,
            interface::commands::product::list_price_adjust,
            interface::commands::product::price_adjustment,
            interface::commands::product::delete_product,
            interface::commands::product::update_product,
            interface::commands::product::list_categories,
            interface::commands::product::delete_category,
            interface::commands::product::create_category,
            interface::commands::product::search_products,
            interface::commands::product::search_price_adjustments,
            interface::commands::transaction::inventory_adjustment,
            interface::commands::transaction::sale_transaction,
            interface::commands::transaction::list_order_details,
            interface::commands::transaction::search_customer_transactions,
            interface::commands::transaction::search_inventory_transactions,
            interface::commands::transaction::set_weekly_limit,
            interface::commands::transaction::get_weekly_limit,
            interface::commands::transaction::get_weekly_spent,
            interface::commands::club::search_customers,
            interface::commands::club::search_club_transactions,
            interface::commands::pos::pos_init,
            interface::commands::legacy_migration::has_legacy_data,
            interface::commands::legacy_migration::do_legacy_data_import,
            interface::commands::parse_pdf::parse_pdf,
            interface::commands::printer::list_printers,
            interface::commands::printer::print_prod_inv_rpt,
            interface::commands::printer::print_cust_bal_rpt,
            interface::commands::printer::print_product_catalog,
        ])
        .on_window_event(|_window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                log::info!("Annex POS is exiting");
            }
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            log::error!("Tauri run failed: {e}");
            std::process::exit(1);
        });
}
