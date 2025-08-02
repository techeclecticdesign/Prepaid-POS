use crate::application::use_cases::printer_usecases::PrinterUseCases;
use crate::common::error::AppError;
use crate::interface::presenters::printer_presenter::PrinterPresenter;
#[derive(Debug)]
enum ReportType {
    Receipt,
}

impl std::str::FromStr for ReportType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "receipt" => Ok(ReportType::Receipt),
            _ => Err(AppError::NotFound(format!("Unknown report: {s}"))),
        }
    }
}

pub struct PrinterController {
    uc: PrinterUseCases,
}

impl PrinterController {
    #[must_use]
    pub const fn new(uc: PrinterUseCases) -> Self {
        Self { uc }
    }

    // Returns a list of printers as DTOs.
    pub fn list_printers(
        &self,
    ) -> Result<Vec<crate::interface::dto::printer_dto::PrinterDto>, AppError> {
        let names = self.uc.list_printers()?;
        Ok(PrinterPresenter::to_dto(names))
    }

    pub fn print_prod_inv_rpt(&self, printer_name: String) -> Result<(), AppError> {
        self.uc.print_prod_inv_rpt(printer_name).map(|_| ())
    }

    pub fn print_cust_bal_rpt(&self, printer_name: String) -> Result<(), AppError> {
        self.uc.print_cust_bal_rpt(printer_name).map(|_| ())
    }

    pub fn print_product_catalog(&self, printer_name: String) -> Result<(), AppError> {
        self.uc.print_product_catalog(printer_name).map(|_| ())
    }
}
