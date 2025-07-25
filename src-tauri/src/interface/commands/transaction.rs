use crate::common::error::AppError;
use crate::interface::controllers::transaction_controller::TransactionController;
use crate::interface::dto::customer_transaction_dto::CustomerTransactionSearchResult;
use crate::interface::dto::customer_tx_detail_dto::CustomerTxDetailDto;
use crate::interface::dto::inventory_transaction_dto::{
    CreateInventoryTransactionDto, InventoryTransactionSearchResult, ReadInventoryTransactionDto,
};
use crate::interface::dto::sale_dto::SaleDto;
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
    dto: SaleDto,
    receipt_printer: String,
) -> Result<i32, AppError> {
    let order_id = controller.sale_transaction(dto.clone(), &receipt_printer)?;
    Ok(order_id)
}

#[tauri::command]
pub fn search_inventory_transactions(
    controller: State<Arc<TransactionController>>,
    page: Option<i32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<InventoryTransactionSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_inventory_transactions(page, date, search)
}

#[tauri::command]
pub fn list_order_details(
    controller: State<'_, Arc<TransactionController>>,
    order_id: i32,
) -> Result<Vec<CustomerTxDetailDto>, AppError> {
    controller.list_order_details(order_id)
}

#[tauri::command]
pub fn search_customer_transactions(
    controller: State<Arc<TransactionController>>,
    page: Option<i32>,
    mdoc: Option<i32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<CustomerTransactionSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_customer_transactions(page, mdoc, date, search)
}
