use crate::common::error::AppError;
use crate::interface::controllers::transaction_controller::TransactionController;
use crate::interface::dto::customer_transaction_dto::{
    CustomerTransactionDto, CustomerTransactionSearchResult,
};
use crate::interface::dto::customer_tx_detail_dto::CreateCustomerTxDetailDto;
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
) -> Result<i32, AppError> {
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
pub fn search_inventory_transactions(
    controller: State<Arc<TransactionController>>,
    page: Option<u32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<InventoryTransactionSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_inventory_transactions(page, date, search)
}

#[tauri::command]
pub fn list_tx_for_customer(
    controller: State<Arc<TransactionController>>,
    customer_mdoc: i32,
) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
    controller.list_tx_for_customer(customer_mdoc)
}

#[tauri::command]
pub fn list_sales(
    controller: State<'_, Arc<TransactionController>>,
) -> Result<Vec<CustomerTransactionDto>, AppError> {
    controller.list_sales()
}

#[tauri::command]
pub fn get_sale(
    controller: State<'_, Arc<TransactionController>>,
    id: i32,
) -> Result<Option<CustomerTransactionDto>, AppError> {
    controller.get_sale(id)
}

#[tauri::command]
pub fn make_sale_line_item(
    controller: State<'_, Arc<TransactionController>>,
    dto: CreateCustomerTxDetailDto,
) -> Result<(), AppError> {
    controller.make_sale_line_item(dto)
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
    page: Option<u32>,
    mdoc: Option<i32>,
    date: Option<String>,
    search: Option<String>,
) -> Result<CustomerTransactionSearchResult, AppError> {
    let page = page.unwrap_or(1);
    controller.search_customer_transactions(page, mdoc, date, search)
}
