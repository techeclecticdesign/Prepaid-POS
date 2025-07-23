use crate::application::use_cases::printer_usecases::PrinterUseCases;
use crate::common::error::AppError;
use crate::interface::presenters::printer_presenter::PrinterPresenter;

pub struct PrinterController {
    uc: PrinterUseCases,
}

impl PrinterController {
    #[must_use]
    pub const fn new(uc: PrinterUseCases) -> Self {
        Self { uc }
    }

    /// Returns a list of printers as DTOs.
    pub fn list_printers(
        &self,
    ) -> Result<Vec<crate::interface::dto::printer_dto::PrinterDto>, AppError> {
        let names = self.uc.list_printers()?;
        Ok(PrinterPresenter::to_dto(names))
    }
}
