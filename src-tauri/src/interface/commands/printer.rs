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
