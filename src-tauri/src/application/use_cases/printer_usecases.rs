use crate::common::error::AppError;
use crate::infrastructure::command_runner::CommandRunner;
use crate::infrastructure::printing::reports::business_receipt::print_business_receipt;
use crate::infrastructure::printing::reports::customer_receipt::print_customer_receipt;
use crate::interface::dto::printer_dto::PrintableSaleDto;
use std::sync::Arc;

pub enum ReportType {
    Receipt,
}

pub struct PrinterUseCases {
    runner: Arc<dyn CommandRunner>,
}

impl PrinterUseCases {
    pub fn new(runner: Arc<dyn CommandRunner>) -> Self {
        Self { runner }
    }

    // List installed printers on Windows by invoking PowerShell.
    pub fn list_printers(&self) -> Result<Vec<String>, AppError> {
        let output = self
            .runner
            .run(
                "powershell",
                &[
                    "-NoProfile",
                    "-Command",
                    "Get-Printer | Select-Object -ExpandProperty Name",
                ],
            )
            .map_err(|e| AppError::Unexpected(format!("Runner failed: {e}")))?;

        if !output.status.success() {
            return Err(AppError::Unexpected(format!(
                "Printer list command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let names = stdout
            .lines()
            .map(str::to_string)
            .filter(|l| !l.trim().is_empty())
            .collect();

        Ok(names)
    }

    // Print both the customer & business receipts.
    pub fn print_receipts(
        &self,
        printable: &PrintableSaleDto,
        printer_name: &str,
        operator_name: &str,
        customer_name: &str,
    ) -> Result<(), AppError> {
        let PrintableSaleDto {
            transaction,
            items,
            balance,
        } = printable;

        // customer copy
        print_customer_receipt(
            transaction,
            items,
            operator_name,
            customer_name,
            printer_name,
        )?;

        // business copy
        print_business_receipt(
            transaction,
            items,
            operator_name,
            customer_name,
            *balance,
            printer_name,
        )?;

        Ok(())
    }
}
