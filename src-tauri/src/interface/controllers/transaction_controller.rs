use crate::application::use_cases::printer_usecases::PrinterUseCases;
use crate::application::use_cases::transaction_usecases::TransactionUseCases;
use crate::common::error::AppError;
use crate::domain::models::customer_transaction::CustomerTransaction;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::inventory_transaction::InventoryTransaction;
use crate::interface::dto::customer_transaction_dto::CustomerTransactionSearchResult;
use crate::interface::dto::customer_tx_detail_dto::CustomerTxDetailDto;
use crate::interface::dto::inventory_transaction_dto::{
    CreateInventoryTransactionDto, InventoryTransactionSearchResult, ReadInventoryTransactionDto,
};
use crate::interface::dto::printer_dto::PrintableLineItem;
use crate::interface::dto::printer_dto::PrintableSaleDto;
use crate::interface::dto::sale_dto::SaleDto;
use crate::interface::presenters::customer_transaction_presenter::CustomerTransactionPresenter;
use crate::interface::presenters::customer_tx_detail_presenter::CustomerTxDetailPresenter;
use crate::interface::presenters::inventory_transaction_presenter::InventoryTransactionPresenter;
use std::sync::{Arc, Mutex};
use validator::Validate;

pub struct TransactionControllerDeps {
    pub inv_repo: Arc<dyn crate::domain::repos::InventoryTransactionRepoTrait>,
    pub cust_tx_repo: Arc<dyn crate::domain::repos::CustomerTransactionRepoTrait>,
    pub cust_tx_detail_repo: Arc<dyn crate::domain::repos::CustomerTxDetailRepoTrait>,
    pub limit_repo: Arc<dyn crate::domain::repos::WeeklyLimitRepoTrait>,
    pub runner: Arc<dyn crate::infrastructure::command_runner::CommandRunner>,
    pub customer_repo: Arc<dyn crate::domain::repos::CustomerRepoTrait>,
    pub prod_repo: Arc<dyn crate::domain::repos::ProductRepoTrait>,
    pub conn: Arc<Mutex<rusqlite::Connection>>,
}

pub struct TransactionController {
    tx_uc: TransactionUseCases,
    printer_uc: PrinterUseCases,
}

impl TransactionController {
    pub fn new(deps: TransactionControllerDeps) -> Self {
        let tx_uc = TransactionUseCases::new(
            deps.inv_repo,
            deps.cust_tx_repo,
            deps.cust_tx_detail_repo,
            deps.limit_repo,
            deps.conn,
        );
        let printer_uc = PrinterUseCases::new(deps.runner, deps.customer_repo, deps.prod_repo);
        Self { tx_uc, printer_uc }
    }

    pub fn inventory_adjustment(
        &self,
        dto: CreateInventoryTransactionDto,
    ) -> Result<ReadInventoryTransactionDto, AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        let tx = InventoryTransaction {
            id: Some(0), // new record, gets auto-assigned by db
            upc: dto.upc,
            quantity_change: dto.quantity_change,
            operator_mdoc: dto.operator_mdoc,
            customer_mdoc: dto.customer_mdoc,
            ref_order_id: dto.ref_order_id,
            reference: dto.reference,
            created_at: None,
        };

        let itx = self.tx_uc.inventory_adjustment(tx)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
    }

    pub fn sale_transaction(&self, dto: SaleDto, printer_name: &str) -> Result<i32, AppError> {
        let cust_tx = CustomerTransaction {
            order_id: 0,
            customer_mdoc: dto.customer_mdoc,
            operator_mdoc: dto.operator_mdoc,
            date: None,
            note: None,
        };

        // build inventory transactions and customer transaction detail lines
        let mut invs = Vec::with_capacity(dto.items.len());
        let mut details = Vec::with_capacity(dto.items.len());
        for item in dto.items {
            invs.push(InventoryTransaction {
                id: None,
                upc: item.upc.clone(),
                quantity_change: item.quantity,
                operator_mdoc: dto.operator_mdoc,
                customer_mdoc: Some(dto.customer_mdoc),
                ref_order_id: None,
                reference: None,
                created_at: None,
            });
            details.push(CustomerTxDetail {
                detail_id: 0,
                order_id: 0,
                upc: item.upc,
                quantity: item.quantity,
                price: item.price,
            });
        }

        let order_id = self.tx_uc.sale_transaction(cust_tx, invs, details);
        if let Ok(order_id) = order_id {
            let printable = self.get_sale_details(order_id)?;
            self.printer_uc.print_receipts(
                &printable,
                printer_name,
                &dto.customer_name,
                &dto.operator_name,
            )?;
        }
        order_id
    }

    pub fn search_inventory_transactions(
        &self,
        page: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<InventoryTransactionSearchResult, AppError> {
        let items = self
            .tx_uc
            .search_inventory_transactions(page, date.clone(), search.clone())?;
        let total = self.tx_uc.count_inventory_transactions(date, search)?;
        let rows = items
            .into_iter()
            .map(|(tx, pname, oname)| {
                InventoryTransactionPresenter::to_search_row(tx, pname, oname)
            })
            .collect();
        Ok(InventoryTransactionSearchResult {
            transactions: rows,
            total_count: total,
        })
    }

    pub fn list_order_details(&self, order_id: i32) -> Result<Vec<CustomerTxDetailDto>, AppError> {
        let dets = self.tx_uc.list_order_details(order_id)?;
        Ok(CustomerTxDetailPresenter::to_dto_list(dets))
    }

    pub fn search_customer_transactions(
        &self,
        page: i32,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<CustomerTransactionSearchResult, AppError> {
        let tuples: Vec<(CustomerTransaction, String, i32)> = self
            .tx_uc
            .search_customer_transactions(page, mdoc, date.clone(), search.clone())?;
        let total = self.tx_uc.count_customer_transactions(mdoc, date, search)?;
        Ok(CustomerTransactionSearchResult {
            items: CustomerTransactionPresenter::to_search_rows(tuples),
            total_count: total,
        })
    }

    pub fn get_sale_details(&self, order_id: i32) -> Result<PrintableSaleDto, AppError> {
        let (tx, details, balance) = self.tx_uc.get_sale_details(order_id)?;
        let items = details
            .into_iter()
            .map(|(d, desc)| PrintableLineItem {
                upc: d.upc,
                desc,
                quantity: d.quantity,
                price: d.price,
            })
            .collect();

        Ok(PrintableSaleDto {
            transaction: tx,
            items,
            balance,
        })
    }

    pub fn get_weekly_limit(&self) -> Result<i32, AppError> {
        self.tx_uc.get_weekly_limit()
    }

    pub fn set_weekly_limit(&self, limit: i32) -> Result<(), AppError> {
        self.tx_uc.set_weekly_limit(limit)
    }

    pub fn get_weekly_spent(&self, customer_mdoc: i32) -> Result<i32, AppError> {
        self.tx_uc.get_weekly_spent(customer_mdoc)
    }
}
