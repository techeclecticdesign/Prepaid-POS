use crate::common::error::AppError;
use crate::interface::controllers::club_controller::ClubController;
use crate::interface::dto::club_import_dto::ClubImportReadDto;
use crate::interface::dto::club_transaction_dto::ClubTransactionReadDto;
use crate::interface::dto::club_transaction_dto::ClubTransactionSearchResult;
use crate::interface::dto::customer_dto::{CustomerReadDto, CustomerSearchResult};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn list_customers(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<CustomerReadDto>, AppError> {
    controller.list_customers()
}

#[tauri::command]
pub fn get_customer(
    controller: State<Arc<ClubController>>,
    mdoc: i32,
) -> Result<Option<CustomerReadDto>, AppError> {
    controller.get_customer(mdoc)
}

#[tauri::command]
pub fn search_customers(
    controller: State<Arc<ClubController>>,
    page: Option<u32>,
    search: Option<String>,
) -> Result<CustomerSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_customers(page, search)
}

#[tauri::command]
pub fn list_club_transactions(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<ClubTransactionReadDto>, AppError> {
    controller.list_club_transactions()
}

#[tauri::command]
pub fn get_club_transaction(
    controller: State<Arc<ClubController>>,
    id: i32,
) -> Result<Option<ClubTransactionReadDto>, AppError> {
    controller.get_club_transaction(id)
}

#[tauri::command]
pub fn list_club_imports(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<ClubImportReadDto>, AppError> {
    controller.list_club_imports()
}

#[tauri::command]
pub fn get_club_import(
    controller: State<Arc<ClubController>>,
    id: i32,
) -> Result<Option<ClubImportReadDto>, AppError> {
    controller.get_club_import(id)
}

#[tauri::command]
pub fn search_club_transactions(
    controller: State<Arc<ClubController>>,
    page: Option<u32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<ClubTransactionSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_club_transactions(page, date, search)
}
