use crate::application::use_cases::transaction_usecases::TransactionUseCases;
use crate::common::error::AppError;
use crate::domain::models::customer_transaction::CustomerTransaction;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::models::inventory_transaction::InventoryTransaction;
use crate::interface::common::date_utils::parse_rfc3339;
use crate::interface::dto::customer_transaction_dto::CustomerTransactionDto;
use crate::interface::dto::customer_tx_detail_dto::{
    CreateCustomerTxDetailDto, CustomerTxDetailDto,
};
use crate::interface::dto::inventory_transaction_dto::{
    CreateInventoryTransactionDto, ReadInventoryTransactionDto,
};
use crate::interface::presenters::customer_tx_detail_presenter::CustomerTxDetailPresenter;
use crate::interface::presenters::inventory_transaction_presenter::InventoryTransactionPresenter;
use std::sync::Arc;
use validator::Validate;

pub struct TransactionController {
    uc: TransactionUseCases,
}

impl TransactionController {
    pub fn new(
        inv_repo: Arc<dyn crate::domain::repos::InventoryTransactionRepoTrait>,
        cust_tx_repo: Arc<dyn crate::domain::repos::CustomerTransactionRepoTrait>,
        detail_repo: Arc<dyn crate::domain::repos::CustomerTxDetailRepoTrait>,
    ) -> Self {
        Self {
            uc: TransactionUseCases::new(inv_repo, cust_tx_repo, detail_repo),
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

    pub fn sale_transaction(
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

        let itx = self.uc.sale_transaction(tx)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
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

    pub fn list_tx_for_customer(
        &self,
        customer_mdoc: i32,
    ) -> Result<Vec<ReadInventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_for_customer(customer_mdoc)?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_sales(&self) -> Result<Vec<CustomerTransaction>, AppError> {
        self.uc.list_sales()
    }

    pub fn get_sale(&self, id: i32) -> Result<Option<CustomerTransaction>, AppError> {
        self.uc.get_sale(id)
    }

    pub fn make_sale(&self, dto: CustomerTransactionDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        let date = match &dto.date {
            Some(s) => Some(parse_rfc3339(s)?),
            None => None,
        };

        let tx = CustomerTransaction {
            order_id: dto.order_id,
            customer_mdoc: dto.customer_mdoc,
            operator_mdoc: dto.operator_mdoc,
            date,
            note: dto.note,
        };

        self.uc.make_sale(tx)
    }

    pub fn make_sale_line_item(&self, dto: CreateCustomerTxDetailDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        let detail = CustomerTxDetail {
            detail_id: 0,
            order_id: dto.order_id,
            upc: dto.upc,
            quantity: dto.quantity,
            price: dto.price,
        };
        self.uc.make_sale_line_item(&detail)
    }

    pub fn list_order_details(&self, order_id: i32) -> Result<Vec<CustomerTxDetailDto>, AppError> {
        let dets = self.uc.list_order_details(order_id)?;
        Ok(CustomerTxDetailPresenter::to_dto_list(dets))
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_customer_tx_detail_repo::MockCustomerTxDetailRepo;
    use crate::test_support::mock_customer_tx_repo::MockCustomerTransactionRepo;
    use crate::test_support::mock_inventory_transaction_repo::MockInventoryTransactionRepo;
    use std::sync::Arc;

    #[test]
    fn controller_smoke_list_transactions() {
        let inv_repo = Arc::new(MockInventoryTransactionRepo::new());
        let cust_tx_repo = Arc::new(MockCustomerTransactionRepo::new());
        let detail_repo = Arc::new(MockCustomerTxDetailRepo::new());
        let ctrl = TransactionController::new(inv_repo.clone(), cust_tx_repo.clone(), detail_repo);
        let out = ctrl
            .list_inv_adjust()
            .expect("list_inv_adjust should succeed");
        assert!(out.is_empty());
    }
}
