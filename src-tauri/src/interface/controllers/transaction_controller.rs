use crate::application::use_cases::transaction_usecases::TransactionUseCases;
use crate::common::error::AppError;
use crate::domain::models::inventory_transaction::InventoryTransaction;
use crate::interface::common::validators::validate_with_optional_dates;
use crate::interface::dto::inventory_transaction_dto::{
    CreateInventoryTransactionDto, ReadInventoryTransactionDto,
};
use crate::interface::presenters::inventory_transaction_presenter::InventoryTransactionPresenter;
use std::sync::Arc;

pub struct TransactionController {
    uc: TransactionUseCases,
}

impl TransactionController {
    pub fn new(inv_repo: Arc<dyn crate::domain::repos::InventoryTransactionRepoTrait>) -> Self {
        Self {
            uc: TransactionUseCases::new(inv_repo),
        }
    }

    pub fn inventory_adjustment(
        &self,
        dto: CreateInventoryTransactionDto,
    ) -> Result<ReadInventoryTransactionDto, AppError> {
        validate_with_optional_dates(&dto).map_err(|e| AppError::Validation(format!("{}", e)))?;
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
        validate_with_optional_dates(&dto).map_err(|e| AppError::Validation(format!("{}", e)))?;
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
        validate_with_optional_dates(&dto).map_err(|e| AppError::Validation(format!("{}", e)))?;
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
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_inventory_transaction_repo::MockInventoryTransactionRepo;
    use std::sync::Arc;

    #[test]
    fn controller_smoke_list_transactions() {
        let inv_repo = Arc::new(MockInventoryTransactionRepo::new());
        let ctrl = TransactionController::new(inv_repo.clone());
        let out = ctrl
            .list_inv_adjust()
            .expect("list_inv_adjust should succeed");
        assert!(out.is_empty());
    }
}
