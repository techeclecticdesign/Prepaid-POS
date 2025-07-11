use crate::common::error::AppError;
use crate::interface::controllers::club_controller::ClubController;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn list_customers(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<crate::interface::dto::customer_dto::CustomerReadDto>, AppError> {
    controller.list_customers()
}

#[tauri::command]
pub fn get_customer(
    controller: State<Arc<ClubController>>,
    mdoc: i32,
) -> Result<Option<crate::interface::dto::customer_dto::CustomerReadDto>, AppError> {
    controller.get_customer(mdoc)
}

#[tauri::command]
pub fn list_club_transactions(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<crate::interface::dto::club_transaction_dto::ClubTransactionReadDto>, AppError> {
    controller.list_club_transactions()
}

#[tauri::command]
pub fn get_club_transaction(
    controller: State<Arc<ClubController>>,
    id: i32,
) -> Result<Option<crate::interface::dto::club_transaction_dto::ClubTransactionReadDto>, AppError> {
    controller.get_club_transaction(id)
}

#[tauri::command]
pub fn list_club_imports(
    controller: State<Arc<ClubController>>,
) -> Result<Vec<crate::interface::dto::club_import_dto::ClubImportReadDto>, AppError> {
    controller.list_club_imports()
}

#[tauri::command]
pub fn get_club_import(
    controller: State<Arc<ClubController>>,
    id: i32,
) -> Result<Option<crate::interface::dto::club_import_dto::ClubImportReadDto>, AppError> {
    controller.get_club_import(id)
}
