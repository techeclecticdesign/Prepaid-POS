use crate::application::use_cases::transaction_usecases::TransactionUseCases;
use crate::common::error::AppError;
use crate::domain::models::customer_transaction::CustomerTransaction;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::inventory_transaction::InventoryTransaction;
use crate::interface::dto::customer_transaction_dto::{
    CustomerTransactionDto, CustomerTransactionSearchResult,
};
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

pub struct TransactionController {
    uc: TransactionUseCases,
}

impl TransactionController {
    pub fn new(
        inv_repo: Arc<dyn crate::domain::repos::InventoryTransactionRepoTrait>,
        cust_tx_repo: Arc<dyn crate::domain::repos::CustomerTransactionRepoTrait>,
        cust_tx_detail_repo: Arc<dyn crate::domain::repos::CustomerTxDetailRepoTrait>,
        conn: Arc<Mutex<rusqlite::Connection>>,
    ) -> Self {
        Self {
            uc: TransactionUseCases::new(inv_repo, cust_tx_repo, cust_tx_detail_repo, conn),
        }
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

        let itx = self.uc.inventory_adjustment(tx)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
    }

    pub fn sale_transaction(&self, dto: SaleDto) -> Result<i32, AppError> {
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

        self.uc.sale_transaction(cust_tx, invs, details)
    }

    pub fn stock_items(
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

        let itx = self.uc.stock_items(tx)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
    }

    pub fn list_inv_adjust_today(&self) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_inv_adjust_today()?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_inv_adjust_operator(
        &self,
        op: i32,
    ) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_inv_adjust_operator(op)?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_inv_adjust(&self) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_inv_adjust()?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn get_transaction(
        &self,
        id: i64,
    ) -> Result<Option<ReadInventoryTransactionDto>, AppError> {
        Ok(self
            .uc
            .get_transaction(id)?
            .map(InventoryTransactionPresenter::to_dto))
    }

    pub fn list_tx_for_product(
        &self,
        upc: String,
    ) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_for_product(upc)?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn search_inventory_transactions(
        &self,
        page: u32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<InventoryTransactionSearchResult, AppError> {
        let items = self
            .uc
            .search_inventory_transactions(page, date.clone(), search.clone())?;
        let total = self.uc.count_inventory_transactions(date, search)?;
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

    pub fn list_tx_for_customer(
        &self,
        customer_mdoc: i32,
    ) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_for_customer(customer_mdoc)?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_sales(&self) -> Result<Vec<CustomerTransactionDto>, AppError> {
        let txs = self.uc.list_sales()?;
        Ok(CustomerTransactionPresenter::to_dto_list(txs))
    }

    pub fn get_sale(&self, id: i32) -> Result<Option<CustomerTransactionDto>, AppError> {
        let opt = self.uc.get_sale(id)?;
        Ok(opt.map(CustomerTransactionPresenter::to_dto))
    }

    pub fn list_order_details(&self, order_id: i32) -> Result<Vec<CustomerTxDetailDto>, AppError> {
        let dets = self.uc.list_order_details(order_id)?;
        Ok(CustomerTxDetailPresenter::to_dto_list(dets))
    }

    pub fn search_customer_transactions(
        &self,
        page: u32,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<CustomerTransactionSearchResult, AppError> {
        let tuples: Vec<(CustomerTransaction, String, i64)> = self
            .uc
            .search_customer_transactions(page, mdoc, date.clone(), search.clone())?;
        let total = self.uc.count_customer_transactions(mdoc, date, search)?;
        Ok(CustomerTransactionSearchResult {
            items: CustomerTransactionPresenter::to_search_rows(tuples),
            total_count: total,
        })
    }

    pub fn get_sale_details(&self, order_id: i32) -> Result<PrintableSaleDto, AppError> {
        let (tx, details, balance) = self.uc.get_sale_details(order_id)?;
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
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_customer_tx_detail_repo::MockCustomerTxDetailRepo;
    use crate::test_support::mock_customer_tx_repo::MockCustomerTransactionRepo;
    use crate::test_support::mock_inventory_transaction_repo::MockInventoryTransactionRepo;
    use std::sync::{Arc, Mutex};

    #[test]
    fn controller_smoke_list_transactions() {
        let inv_repo = Arc::new(MockInventoryTransactionRepo::new());
        let cust_tx_repo = Arc::new(MockCustomerTransactionRepo::new());
        let cust_tx_detail_repo = Arc::new(MockCustomerTxDetailRepo::new());
        let ctrl = TransactionController::new(
            inv_repo.clone(),
            cust_tx_repo.clone(),
            cust_tx_detail_repo,
            Arc::new(Mutex::new(rusqlite::Connection::open_in_memory().unwrap())),
        );
        let out = ctrl
            .list_inv_adjust()
            .expect("list_inv_adjust should succeed");
        assert!(out.is_empty());
    }
}
