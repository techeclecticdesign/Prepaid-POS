use crate::common::error::AppError;
use crate::interface::controllers::transaction_controller::TransactionController;
use crate::interface::dto::inventory_transaction_dto::InventoryTransactionDto;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn inventory_adjustment(
    controller: State<Arc<TransactionController>>,
    operator_mdoc: i32,
    upc: i64,
    quantity_change: i32,
) -> Result<InventoryTransactionDto, AppError> {
    controller.inventory_adjustment(operator_mdoc, upc, quantity_change)
}

#[tauri::command]
pub fn sale_transaction(
    controller: State<Arc<TransactionController>>,
    operator_mdoc: i32,
    customer_mdoc: i32,
    upc: i64,
    quantity_change: i32,
) -> Result<InventoryTransactionDto, AppError> {
    controller.sale_transaction(operator_mdoc, customer_mdoc, upc, quantity_change)
}

#[tauri::command]
pub fn stock_items(
    controller: State<Arc<TransactionController>>,
    operator_mdoc: i32,
    upc: i64,
    quantity_change: i32,
) -> Result<InventoryTransactionDto, AppError> {
    controller.stock_items(operator_mdoc, upc, quantity_change)
}

#[tauri::command]
pub fn list_inv_adjust_today(
    controller: State<Arc<TransactionController>>,
) -> Result<Vec<InventoryTransactionDto>, AppError> {
    controller.list_inv_adjust_today()
}

#[tauri::command]
pub fn list_inv_adjust_operator(
    controller: State<Arc<TransactionController>>,
    operator_mdoc: i32,
) -> Result<Vec<InventoryTransactionDto>, AppError> {
    controller.list_inv_adjust_operator(operator_mdoc)
}

#[tauri::command]
pub fn list_inv_adjust(
    controller: State<Arc<TransactionController>>,
) -> Result<Vec<InventoryTransactionDto>, AppError> {
    controller.list_inv_adjust()
}

#[tauri::command]
pub fn get_transaction(
    controller: State<Arc<TransactionController>>,
    id: i64,
) -> Result<Option<InventoryTransactionDto>, AppError> {
    controller.get_transaction(id)
}
#[tauri::command]
pub fn list_tx_for_product(
    controller: State<Arc<TransactionController>>,
    upc: i64,
) -> Result<Vec<InventoryTransactionDto>, AppError> {
    controller.list_tx_for_product(upc)
}
#[tauri::command]
pub fn list_tx_for_customer(
    controller: State<Arc<TransactionController>>,
    customer_mdoc: i32,
) -> Result<Vec<InventoryTransactionDto>, AppError> {
    controller.list_tx_for_customer(customer_mdoc)
}
