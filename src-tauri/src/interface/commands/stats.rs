use crate::common::error::AppError;
use crate::interface::controllers::stats_controller::StatsController;
use crate::interface::dto::stats_dto::StatsDto;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub fn get_stats(controller: State<Arc<StatsController>>) -> Result<StatsDto, AppError> {
    controller.get_stats()
}
