use crate::application::use_cases::transaction_usecases::TransactionUseCases;
use crate::common::error::AppError;
use crate::interface::dto::inventory_transaction_dto::InventoryTransactionDto;
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
        operator_mdoc: i32,
        upc: i64,
        quantity_change: i32,
    ) -> Result<InventoryTransactionDto, AppError> {
        let itx = self
            .uc
            .inventory_adjustment(operator_mdoc, None, upc, quantity_change)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
    }

    pub fn sale_transaction(
        &self,
        operator_mdoc: i32,
        customer_mdoc: i32,
        upc: i64,
        quantity_change: i32,
    ) -> Result<InventoryTransactionDto, AppError> {
        let itx =
            self.uc
                .sale_transaction(operator_mdoc, Some(customer_mdoc), upc, quantity_change)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
    }

    pub fn stock_items(
        &self,
        operator_mdoc: i32,
        upc: i64,
        quantity_change: i32,
    ) -> Result<InventoryTransactionDto, AppError> {
        let itx = self.uc.stock_items(operator_mdoc, upc, quantity_change)?;
        Ok(InventoryTransactionPresenter::to_dto(itx))
    }

    pub fn list_inv_adjust_today(&self) -> Result<Vec<InventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_inv_adjust_today()?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_inv_adjust_operator(
        &self,
        op: i32,
    ) -> Result<Vec<InventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_inv_adjust_operator(op)?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_inv_adjust(&self) -> Result<Vec<InventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_inv_adjust()?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn get_transaction(&self, id: i64) -> Result<Option<InventoryTransactionDto>, AppError> {
        Ok(self
            .uc
            .get_transaction(id)?
            .map(InventoryTransactionPresenter::to_dto))
    }

    pub fn list_tx_for_product(&self, upc: i64) -> Result<Vec<InventoryTransactionDto>, AppError> {
        let itxs = self.uc.list_for_product(upc)?;
        Ok(InventoryTransactionPresenter::to_dto_list(itxs))
    }

    pub fn list_tx_for_customer(
        &self,
        customer_mdoc: i32,
    ) -> Result<Vec<InventoryTransactionDto>, AppError> {
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
