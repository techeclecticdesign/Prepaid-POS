use crate::common::error::AppError;
use crate::domain::report_models::club_import_report::ClubTransactionRow;
use crate::domain::repos::ClubImportRepoTrait;
use crate::domain::repos::ClubTransactionRepoTrait;
use crate::domain::repos::CustomerRepoTrait;
use crate::domain::repos::CustomerTransactionRepoTrait;
use crate::domain::repos::CustomerTxDetailRepoTrait;
use crate::domain::repos::ProductRepoTrait;
use crate::infrastructure::command_runner::CommandRunner;
use crate::infrastructure::printing::reports::business_receipt::print_business_receipt;
use crate::infrastructure::printing::reports::club_imports::print_club_import_report;
use crate::infrastructure::printing::reports::customer_balance_report::print_customer_balance_report;
use crate::infrastructure::printing::reports::customer_receipt::print_customer_receipt;
use crate::infrastructure::printing::reports::print_daily_sales::print_daily_sales;
use crate::infrastructure::printing::reports::prod_inv_report::print_inventory_report;
use crate::infrastructure::printing::reports::product_catalog::print_product_catalog_report;
use crate::infrastructure::printing::reports::product_sales::print_product_sales;
use crate::infrastructure::printing::reports::sales_detail_report::print_sales_detail_report;
use crate::interface::dto::printer_dto::PrintableSaleDto;
use chrono::NaiveDateTime;
use std::sync::Arc;

pub enum ReportType {
    Receipt,
}

pub struct PrinterUseCases {
    runner: Arc<dyn CommandRunner>,
    customer_repo: Arc<dyn CustomerRepoTrait>,
    product_repo: Arc<dyn ProductRepoTrait>,
    cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
    cust_tx_detail_repo: Arc<dyn crate::domain::repos::CustomerTxDetailRepoTrait>,
    club_import_repo: Arc<dyn ClubImportRepoTrait>,
    club_tx_repo: Arc<dyn ClubTransactionRepoTrait>,
}

impl PrinterUseCases {
    pub fn new(
        runner: Arc<dyn CommandRunner>,
        customer_repo: Arc<dyn CustomerRepoTrait>,
        product_repo: Arc<dyn ProductRepoTrait>,
        cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
        cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
        club_import_repo: Arc<dyn ClubImportRepoTrait>,
        club_tx_repo: Arc<dyn ClubTransactionRepoTrait>,
    ) -> Self {
        Self {
            runner,
            customer_repo,
            product_repo,
            cust_tx_repo,
            cust_tx_detail_repo,
            club_import_repo,
            club_tx_repo,
        }
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

    pub fn print_prod_inv_rpt(&self, printer_name: String) -> Result<(), AppError> {
        let rows = self.product_repo.report_by_category()?;
        let total_amount = self.customer_repo.sum_all_balances()?;
        let product_totals = self.product_repo.get_inventory_totals()?;

        print_inventory_report(&rows, product_totals, total_amount, &printer_name)?;
        Ok(())
    }

    pub fn print_cust_bal_rpt(&self, printer_name: String) -> Result<(), AppError> {
        let data = self.customer_repo.list_customer_accounts()?;
        let total_amount = self.customer_repo.sum_all_balances()?;

        print_customer_balance_report(&data, total_amount, &printer_name)?;
        Ok(())
    }

    pub fn print_product_catalog(&self, printer_name: String) -> Result<(), AppError> {
        // fetch the same rows as inventory report
        let rows = self.product_repo.list()?;
        print_product_catalog_report(&rows, &printer_name)?;
        Ok(())
    }

    pub fn print_sales_detail_report(
        &self,
        start_date: chrono::NaiveDateTime,
        end_date: chrono::NaiveDateTime,
        printer_name: String,
    ) -> Result<(), AppError> {
        let data = self
            .cust_tx_repo
            .get_sales_details_data(start_date, end_date)?;
        let total_amount = self.customer_repo.sum_all_balances()?;
        print_sales_detail_report(&data, start_date, end_date, total_amount, &printer_name)?;
        Ok(())
    }

    pub fn sales_by_category(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
        printer_name: String,
    ) -> Result<(), AppError> {
        let data = self.cust_tx_detail_repo.sales_by_category(start, end)?;
        let sales_totals = self.cust_tx_detail_repo.get_sales_totals(start, end)?;
        let total_amount = self.customer_repo.sum_all_balances()?;
        print_product_sales(&data, start, end, sales_totals, total_amount, &printer_name)?;
        Ok(())
    }

    pub fn sales_by_day(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
        printer_name: String,
    ) -> Result<(), AppError> {
        let data = self.cust_tx_detail_repo.sales_by_day(start, end)?;
        let total_amount = self.customer_repo.sum_all_balances()?;
        print_daily_sales(&data, start, end, total_amount, &printer_name)?;
        Ok(())
    }

    pub fn print_club_import(
        &self,
        id: i32,
        start_date: NaiveDateTime,
        printer_name: String,
    ) -> Result<(), AppError> {
        let tx = self
            .club_tx_repo
            .get_by_import_id_with_total(id, Some(start_date))?;
        let import = self
            .club_import_repo
            .get_by_id(id)?
            .ok_or_else(|| AppError::NotFound("Import not found".into()))?;
        let total_amount = self.customer_repo.sum_all_balances()?;
        let tx_rows: Vec<ClubTransactionRow> = tx
            .into_iter()
            .map(|row| ClubTransactionRow {
                running_total: row.running_total,
                tx: row,
            })
            .collect();
        let period_totals = self.club_tx_repo.get_period_sums_for_import(id)?;

        print_club_import_report(
            &import,
            &tx_rows,
            total_amount,
            &period_totals,
            &printer_name,
        )?;

        Ok(())
    }
}
