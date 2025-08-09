use crate::common::error::AppError;
use crate::domain::models::ClubImport;
use crate::interface::controllers::club_controller::ClubController;
use crate::interface::dto::club_transaction_dto::ClubTransactionSearchResult;
use crate::interface::dto::customer_dto::CustomerSearchResult;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn search_customers(
    controller: State<Arc<ClubController>>,
    page: Option<i32>,
    search: Option<String>,
) -> Result<CustomerSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_customers(page, search)
}

#[tauri::command]
pub fn search_club_transactions(
    controller: State<Arc<ClubController>>,
    page: Option<i32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<ClubTransactionSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_club_transactions(page, date, search)
}

#[tauri::command]
pub fn list_club_imports(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<ClubImport>, AppError> {
    controller.list_club_imports()
}
