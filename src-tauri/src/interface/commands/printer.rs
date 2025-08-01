use crate::common::error::AppError;
use crate::interface::controllers::printer_controller::PrinterController;
use crate::interface::dto::printer_dto::PrinterDto;
use std::sync::Arc;
use tauri::State;

// List the installed printers on windows
#[tauri::command]
pub fn list_printers(
    controller: State<'_, Arc<PrinterController>>,
) -> Result<Vec<PrinterDto>, AppError> {
    controller.list_printers()
}

#[tauri::command]
pub fn print_prod_inv_rpt(
    controller: State<'_, Arc<PrinterController>>,
    printer_name: String,
) -> Result<(), AppError> {
    controller.print_prod_inv_rpt(printer_name)
}

#[tauri::command]
pub fn print_cust_bal_rpt(
    controller: State<'_, Arc<PrinterController>>,
    printer_name: String,
) -> Result<(), AppError> {
    controller.print_cust_bal_rpt(printer_name)
}
