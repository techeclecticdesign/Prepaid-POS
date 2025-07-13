use crate::common::error::AppError;
use crate::interface::controllers::transaction_controller::TransactionController;
use crate::interface::dto::inventory_transaction_dto::{
    CreateInventoryTransactionDto, ReadInventoryTransactionDto,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn inventory_adjustment(
    controller: State<Arc<TransactionController>>,
    dto: CreateInventoryTransactionDto,
) -> Result<ReadInventoryTransactionDto, AppError> {
    controller.inventory_adjustment(dto)
}

#[tauri::command]
pub fn sale_transaction(
    controller: State<Arc<TransactionController>>,
    dto: CreateInventoryTransactionDto,
) -> Result<ReadInventoryTransactionDto, AppError> {
    controller.sale_transaction(dto)
}

#[tauri::command]
pub fn stock_items(
    controller: State<Arc<TransactionController>>,
    dto: CreateInventoryTransactionDto,
) -> Result<ReadInventoryTransactionDto, AppError> {
    controller.stock_items(dto)
}

#[tauri::command]
pub fn list_inv_adjust_today(
    controller: State<Arc<TransactionController>>,
) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
    controller.list_inv_adjust_today()
}

#[tauri::command]
pub fn list_inv_adjust_operator(
    controller: State<Arc<TransactionController>>,
    operator_mdoc: i32,
) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
    controller.list_inv_adjust_operator(operator_mdoc)
}

#[tauri::command]
pub fn list_inv_adjust(
    controller: State<Arc<TransactionController>>,
) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
    controller.list_inv_adjust()
}

#[tauri::command]
pub fn get_transaction(
    controller: State<Arc<TransactionController>>,
    id: i64,
) -> Result<Option<ReadInventoryTransactionDto>, AppError> {
    controller.get_transaction(id)
}
#[tauri::command]
pub fn list_tx_for_product(
    controller: State<Arc<TransactionController>>,
    upc: String,
) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
    controller.list_tx_for_product(upc)
}
#[tauri::command]
pub fn list_tx_for_customer(
    controller: State<Arc<TransactionController>>,
    customer_mdoc: i32,
) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
    controller.list_tx_for_customer(customer_mdoc)
}
