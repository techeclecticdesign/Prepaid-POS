use crate::common::error::AppError;
use crate::domain::models::InventoryTransaction;
use crate::domain::repos::InventoryTransactionRepoTrait;
use log::{error, info};
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
        mut tx: InventoryTransaction,
    ) -> Result<InventoryTransaction, AppError> {
        tx.created_at = Some(chrono::Utc::now().naive_utc());

        let res = self.inv_repo.create(&tx);
        match &res {
            Ok(()) => info!(
                "inventory adjustment: upc={} change={} operator={} ",
                tx.upc, tx.quantity_change, tx.operator_mdoc
            ),
            Err(e) => error!(
                "inventory adjustment error: upc={} operator={} error={}",
                tx.upc, tx.operator_mdoc, e
            ),
        };
        res.map(|_| tx)
    }

    pub fn sale_transaction(
        &self,
        mut tx: InventoryTransaction,
    ) -> Result<InventoryTransaction, AppError> {
        tx.created_at = Some(chrono::Utc::now().naive_utc());
        self.inv_repo.create(&tx)?;
        Ok(tx)
    }

    pub fn stock_items(
        &self,
        mut tx: InventoryTransaction,
    ) -> Result<InventoryTransaction, AppError> {
        if tx.quantity_change <= 0 {
            return Err(AppError::Unexpected("quantity_change must be > 0".into()));
        }
        tx.customer_mdoc = None;
        tx.ref_order_id = None;
        tx.reference = Some("Stock Addition".to_string());

        let out = self.inventory_adjustment(tx)?;
        info!(
            "stock items: upc={} added={} operator={}",
            out.upc, out.quantity_change, out.operator_mdoc
        );
        Ok(out)
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

    pub fn list_for_product(&self, upc: String) -> Result<Vec<InventoryTransaction>, AppError> {
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

    impl Default for InventoryTransaction {
        fn default() -> Self {
            Self {
                id: Some(0),
                upc: "000000000000".into(),
                quantity_change: 0,
                operator_mdoc: 0,
                customer_mdoc: None,
                ref_order_id: None,
                reference: None,
                created_at: None,
            }
        }
    }

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
            mdoc: 10,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: "000000000555".into(),
            desc: "Item".into(),
            category: "Cat".into(),
            price: 1000,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })?;

        // inventory adjust
        let itx1 = uc.inventory_adjustment(InventoryTransaction {
            operator_mdoc: 10,
            upc: "000000000555".into(),
            quantity_change: 5,
            ..Default::default()
        })?;
        assert_eq!(itx1.upc, "000000000555");
        assert_eq!(itx1.quantity_change, 5);

        // sale transaction
        let itx2 = uc.sale_transaction(InventoryTransaction {
            operator_mdoc: 10,
            customer_mdoc: Some(20),
            upc: "000000000555".into(),
            quantity_change: -2,
            ..Default::default()
        })?;

        assert_eq!(itx2.customer_mdoc, Some(20));
        assert_eq!(itx2.quantity_change, -2);

        // stock items (positive only)
        let itx3 = uc.stock_items(InventoryTransaction {
            operator_mdoc: 10,
            customer_mdoc: Some(20),
            upc: "000000000555".into(),
            quantity_change: 3,
            ..Default::default()
        })?;

        assert_eq!(itx3.quantity_change, 3);

        // 0 or negative stock fails
        let err = uc
            .stock_items(InventoryTransaction {
                operator_mdoc: 10,
                upc: "000000000555".into(),
                quantity_change: 0,
                ..Default::default()
            })
            .unwrap_err();
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
            mdoc: 1,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        op_repo.create(&Operator {
            mdoc: 2,
            name: "Op2".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: "000000000001".into(),
            desc: "X".into(),
            category: "Y".into(),
            price: 500,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })?;

        // create two adjustments
        uc.inventory_adjustment(InventoryTransaction {
            operator_mdoc: 1,
            upc: "000000000001".into(),
            quantity_change: 1,
            ..Default::default()
        })?;

        uc.inventory_adjustment(InventoryTransaction {
            operator_mdoc: 2,
            upc: "000000000001".into(),
            quantity_change: 2,
            ..Default::default()
        })?;

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
