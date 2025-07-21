use crate::interface::dto::printer_dto::PrinterDto;

// Converts raw printer names into vec of DTOs
pub struct PrinterPresenter;

impl PrinterPresenter {
    pub fn to_dto(names: Vec<String>) -> Vec<PrinterDto> {
        names.into_iter().map(|name| PrinterDto { name }).collect()
    }
}
