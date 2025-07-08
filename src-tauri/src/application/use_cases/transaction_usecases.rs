use crate::common::error::AppError;
use crate::domain::models::InventoryTransaction;
use crate::domain::repos::InventoryTransactionRepoTrait;
use std::sync::Arc;

pub struct TransactionUseCases {
    inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
}

impl TransactionUseCases {
    pub fn new(inv_repo: Arc<dyn InventoryTransactionRepoTrait>) -> Self {
        Self { inv_repo }
    }

    pub fn inventory_adjustment(
        &self,
        operator_mdoc: i32,
        customer_mdoc: Option<i32>,
        upc: i64,
        quantity_change: i32,
    ) -> Result<InventoryTransaction, AppError> {
        let tx = InventoryTransaction {
            id: 0,
            upc,
            quantity_change,
            operator_mdoc,
            customer_mdoc,
            ref_order_id: None,
            reference: Some("Operator Adjustment".to_string()),
            created_at: Some(chrono::Utc::now().naive_utc()),
        };

        self.inv_repo.create(&tx)?;
        Ok(tx)
    }

    pub fn sale_transaction(
        &self,
        operator_mdoc: i32,
        customer_mdoc: Option<i32>,
        upc: i64,
        quantity_change: i32,
    ) -> Result<InventoryTransaction, AppError> {
        let tx = InventoryTransaction {
            id: 0,
            upc,
            quantity_change,
            operator_mdoc,
            customer_mdoc,
            ref_order_id: None,
            reference: Some("Operator Adjustment".to_string()),
            created_at: Some(chrono::Utc::now().naive_utc()),
        };
        self.inv_repo.create(&tx)?;
        Ok(tx)
    }

    pub fn stock_items(
        &self,
        operator_mdoc: i32,
        upc: i64,
        quantity_change: i32,
    ) -> Result<InventoryTransaction, AppError> {
        if quantity_change <= 0 {
            return Err(AppError::Unexpected("quantity_change must be > 0".into()));
        }
        self.inventory_adjustment(operator_mdoc, None, upc, quantity_change)
    }

    pub fn list_inv_adjust_today(&self) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_today()
    }

    pub fn list_inv_adjust_operator(&self, op: i32) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_operator(op)
    }

    pub fn list_inv_adjust(&self) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list()
    }

    pub fn get_transaction(&self, id: i64) -> Result<Option<InventoryTransaction>, AppError> {
        self.inv_repo.get_by_id(id)
    }

    pub fn list_for_product(&self, upc: i64) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_product(upc)
    }

    pub fn list_for_customer(
        &self,
        customer_mdoc: i32,
    ) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_customer(customer_mdoc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{Operator, Product};
    use crate::domain::repos::{OperatorRepoTrait, ProductRepoTrait};
    use crate::infrastructure::db::create_connection;
    use crate::infrastructure::repos::{
        SqliteInventoryTransactionRepo, SqliteOperatorRepo, SqliteProductRepo,
    };
    use std::sync::Arc;

    fn make_use_cases() -> (
        TransactionUseCases,
        Arc<dyn OperatorRepoTrait>,
        Arc<dyn ProductRepoTrait>,
    ) {
        let conn = Arc::new(create_connection(":memory:").unwrap());

        // build all three repos
        let op_repo: Arc<dyn OperatorRepoTrait> =
            Arc::new(SqliteOperatorRepo::new(Arc::clone(&conn)));
        let prod_repo: Arc<dyn ProductRepoTrait> =
            Arc::new(SqliteProductRepo::new(Arc::clone(&conn)));
        let inv_repo = Arc::new(SqliteInventoryTransactionRepo::new(Arc::clone(&conn)));

        let uc = TransactionUseCases::new(inv_repo);
        (uc, op_repo, prod_repo)
    }

    #[test]
    fn inventory_and_sale_and_stock_flows() -> anyhow::Result<()> {
        let (uc, op_repo, prod_repo) = make_use_cases();

        // seed FK tables
        op_repo.create(&Operator {
            id: 10,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: 555,
            desc: "Item".into(),
            category: "Cat".into(),
            price: 1000,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })?;

        // inventory adjust
        let itx1 = uc.inventory_adjustment(10, None, 555, 5)?;
        assert_eq!(itx1.upc, 555);
        assert_eq!(itx1.quantity_change, 5);

        // sale transaction
        let itx2 = uc.sale_transaction(10, Some(20), 555, -2)?;

        assert_eq!(itx2.customer_mdoc, Some(20));
        assert_eq!(itx2.quantity_change, -2);

        // stock items (positive only)
        let itx3 = uc.stock_items(10, 555, 3)?;

        assert_eq!(itx3.quantity_change, 3);

        // negative stock fails
        let err = uc.stock_items(10, 555, 0).unwrap_err();
        assert!(err.to_string().contains("must be > 0"));

        // listing all
        let all = uc.list_inv_adjust()?;
        assert_eq!(all.len(), 3);

        Ok(())
    }

    #[test]
    fn list_filters() -> anyhow::Result<()> {
        let (uc, op_repo, prod_repo) = make_use_cases();

        // seed FK tables
        op_repo.create(&Operator {
            id: 1,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        op_repo.create(&Operator {
            id: 2,
            name: "Op2".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: 1,
            desc: "X".into(),
            category: "Y".into(),
            price: 500,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })?;

        // create two adjustments
        uc.inventory_adjustment(1, None, 1, 1)?;
        uc.inventory_adjustment(2, None, 1, 2)?;

        let all = uc.list_inv_adjust()?;
        assert_eq!(all.len(), 2);

        let op1 = uc.list_inv_adjust_operator(1)?;
        assert_eq!(op1.len(), 1);

        // nothing for today filter if date logic differs
        let today = uc.list_inv_adjust_today()?;
        assert!(today.len() >= 2);
        Ok(())
    }
}
