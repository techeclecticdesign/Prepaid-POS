use crate::common::error::AppError;
use crate::domain::models::{CustomerTransaction, CustomerTxDetail, InventoryTransaction};
use crate::domain::repos::CustomerTransactionRepoTrait;
use crate::domain::repos::CustomerTxDetailRepoTrait;
use crate::domain::repos::InventoryTransactionRepoTrait;
use log::{error, info};
use std::sync::Arc;

pub struct TransactionUseCases {
    inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
    cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
    detail_repo: Arc<dyn crate::domain::repos::CustomerTxDetailRepoTrait>,
}

impl TransactionUseCases {
    pub fn new(
        inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
        cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
        detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
    ) -> Self {
        Self {
            inv_repo,
            cust_tx_repo,
            detail_repo,
        }
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

    pub fn search_inventory_transactions(
        &self,
        page: u32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(InventoryTransaction, String, String)>, AppError> {
        let limit = 10;
        let offset = (page.saturating_sub(1) as i64) * limit;
        self.inv_repo.search(limit, offset, date, search)
    }

    pub fn count_inventory_transactions(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<u32, AppError> {
        self.inv_repo.count(date, search).map(|c| c as u32)
    }

    pub fn list_for_customer(
        &self,
        customer_mdoc: i32,
    ) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_customer(customer_mdoc)
    }

    pub fn make_sale(&self, mut tx: CustomerTransaction) -> Result<(), AppError> {
        // stamp entry time
        tx.date = Some(chrono::Utc::now().naive_utc());
        // If a specific order_id is requested, ensure it's unique
        if tx.order_id > 0 && self.cust_tx_repo.get(tx.order_id)?.is_some() {
            return Err(AppError::Unexpected(format!(
                "CustomerTransaction with order_id={} already exists",
                tx.order_id
            )));
        }
        // delegate to repo
        let res = self.cust_tx_repo.create(&tx);
        match &res {
            Ok(()) => info!(
                "customer transaction created: order_id={} cust={} op={}",
                tx.order_id, tx.customer_mdoc, tx.operator_mdoc
            ),
            Err(e) => error!(
                "customer transaction error: cust={} op={} error={}",
                tx.customer_mdoc, tx.operator_mdoc, e
            ),
        }
        res
    }

    pub fn list_sales(&self) -> Result<Vec<CustomerTransaction>, AppError> {
        self.cust_tx_repo.list()
    }

    pub fn get_sale(&self, order_id: i32) -> Result<Option<CustomerTransaction>, AppError> {
        self.cust_tx_repo.get(order_id)
    }

    pub fn make_sale_line_item(&self, detail: &CustomerTxDetail) -> Result<(), AppError> {
        let res = self.detail_repo.create(detail);
        match &res {
            Ok(()) => info!(
                "detail created: order_id={} upc={} qty={}",
                detail.order_id, detail.upc, detail.quantity
            ),
            Err(e) => error!(
                "detail create error: order_id={} upc={} error={}",
                detail.order_id, detail.upc, e
            ),
        }
        res
    }

    pub fn list_order_details(&self, order_id: i32) -> Result<Vec<CustomerTxDetail>, AppError> {
        self.detail_repo.list_by_order(order_id)
    }

    pub fn search_customer_transactions(
        &self,
        page: u32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<CustomerTransaction>, AppError> {
        let limit = 10;
        let offset = (page.saturating_sub(1) as i64) * limit;
        self.cust_tx_repo.search(limit, offset, date, search)
    }

    pub fn count_customer_transactions(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<u32, AppError> {
        self.cust_tx_repo.count(date, search).map(|c| c as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::CustomerTransaction;
    use crate::domain::models::{Operator, Product};
    use crate::domain::repos::{OperatorRepoTrait, ProductRepoTrait};
    use crate::infrastructure::db::create_connection;
    use crate::infrastructure::repos::{
        SqliteCustomerTransactionRepo, SqliteCustomerTxDetailRepo, SqliteInventoryTransactionRepo,
        SqliteOperatorRepo, SqliteProductRepo,
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
        Arc<dyn CustomerTransactionRepoTrait>,
    ) {
        let conn = Arc::new(create_connection(":memory:").unwrap());

        {
            let db = conn.lock().unwrap();
            db.execute_batch(
                "
            CREATE TABLE IF NOT EXISTS customer_transactions (
                order_id     INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_mdoc INTEGER NOT NULL,
                operator_mdoc INTEGER NOT NULL,
                date   TEXT,
                note         TEXT,
                date_cancelled TEXT
            );
            ",
            )
            .unwrap();
        }

        {
            let db = conn.lock().unwrap();
            db.execute_batch(
                "
           CREATE TABLE IF NOT EXISTS customer_tx_detail (
               detail_id   INTEGER PRIMARY KEY AUTOINCREMENT,
               order_id    INTEGER NOT NULL,
               upc         TEXT NOT NULL,
               quantity    INTEGER NOT NULL,
               price       INTEGER NOT NULL
           );
       ",
            )
            .unwrap();
        }

        // build all three repos
        let op_repo: Arc<dyn OperatorRepoTrait> =
            Arc::new(SqliteOperatorRepo::new(Arc::clone(&conn)));
        let prod_repo: Arc<dyn ProductRepoTrait> =
            Arc::new(SqliteProductRepo::new(Arc::clone(&conn)));
        let inv_repo = Arc::new(SqliteInventoryTransactionRepo::new(Arc::clone(&conn)));
        let cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait> =
            Arc::new(SqliteCustomerTransactionRepo::new(Arc::clone(&conn)));
        let detail_repo = Arc::new(SqliteCustomerTxDetailRepo::new(Arc::clone(&conn)));

        let uc = TransactionUseCases::new(inv_repo, Arc::clone(&cust_tx_repo), detail_repo);
        (uc, op_repo, prod_repo, cust_tx_repo)
    }

    #[test]
    fn inventory_and_sale_and_stock_flows() -> anyhow::Result<()> {
        let (uc, op_repo, prod_repo, _cust_tx_repo) = make_use_cases();

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
        let (uc, op_repo, prod_repo, _cust_tx_repo) = make_use_cases();

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

    #[test]
    fn make_sale_explicit_order_id_uniqueness() -> anyhow::Result<()> {
        let (uc, op_repo, _prod_repo, _cust_tx_repo) = make_use_cases();

        let details_before = uc.list_order_details(42)?;
        assert!(details_before.is_empty());

        // Seed an operator so FK passes if needed
        op_repo.create(&Operator {
            mdoc: 1,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;

        // Create first transaction with order_id=42
        let tx1 = CustomerTransaction {
            order_id: 42,
            customer_mdoc: 1,
            operator_mdoc: 1,
            date: None,
            note: Some("First".into()),
        };
        uc.make_sale(tx1)?;

        // And create a detail for it
        let detail = crate::domain::models::customer_tx_detail::CustomerTxDetail {
            detail_id: 0,
            order_id: 42,
            upc: "00000001".into(),
            quantity: 1,
            price: 100,
        };
        uc.make_sale_line_item(&detail)?;
        let dets = uc.list_order_details(42)?;
        assert_eq!(dets.len(), 1);

        // Attempt to create second transaction with same order_id
        let tx2 = CustomerTransaction {
            order_id: 42,
            customer_mdoc: 2,
            operator_mdoc: 1,
            date: None,
            note: Some("Duplicate".into()),
        };
        let err = uc.make_sale(tx2).unwrap_err();
        assert!(
            err.to_string().contains("order_id=42 already exists"),
            "unexpected error: {err}"
        );
        Ok(())
    }
}
