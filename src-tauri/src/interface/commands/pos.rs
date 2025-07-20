use crate::common::error::AppError;
use crate::interface::controllers::pos_controller::PosController;
use crate::interface::dto::pos_dto::PosDto;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn pos_init(controller: State<Arc<PosController>>) -> Result<PosDto, AppError> {
    controller.pos_init().map(|(products, customers)| PosDto {
        products,
        customers,
    })
}
