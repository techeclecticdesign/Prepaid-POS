use crate::application::common::db::atomic_tx;
use crate::common::error::AppError;
use crate::domain::models::{CustomerTransaction, CustomerTxDetail, InventoryTransaction};
use crate::domain::repos::CustomerTransactionRepoTrait;
use crate::domain::repos::CustomerTxDetailRepoTrait;
use crate::domain::repos::InventoryTransactionRepoTrait;
use log::{error, info};
use std::sync::{Arc, Mutex};

pub struct TransactionUseCases {
    inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
    cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
    cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl TransactionUseCases {
    pub fn new(
        inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
        cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
        cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
        conn: Arc<Mutex<rusqlite::Connection>>,
    ) -> Self {
        Self {
            inv_repo,
            cust_tx_repo,
            cust_tx_detail_repo,
            conn,
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
        cust_tx: CustomerTransaction,
        invs: Vec<InventoryTransaction>,
        mut details: Vec<CustomerTxDetail>,
    ) -> Result<i32, AppError> {
        atomic_tx(&self.conn, |tx| {
            self.cust_tx_repo.create_with_tx(&cust_tx, tx)?;

            let order_id = tx.last_insert_rowid() as i32;

            for inv in &invs {
                self.inv_repo.create_with_tx(inv, tx)?;
            }

            // insert each detail, fixing its order_id FK
            for det in &mut details {
                det.order_id = order_id;
                self.cust_tx_detail_repo.create_with_tx(det, tx)?;
            }

            Ok(order_id)
        })
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
        let res = self.cust_tx_detail_repo.create(detail);
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

    pub fn list_order_details(
        &self,
        order_id: i32,
    ) -> Result<Vec<(CustomerTxDetail, String)>, AppError> {
        self.cust_tx_detail_repo.list_by_order(order_id)
    }

    pub fn search_customer_transactions(
        &self,
        page: u32,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(CustomerTransaction, String, i64)>, AppError> {
        let limit = 10;
        let offset = (page.saturating_sub(1) as i64) * limit;
        self.cust_tx_repo.search(limit, offset, mdoc, date, search)
    }

    pub fn count_customer_transactions(
        &self,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<u32, AppError> {
        self.cust_tx_repo
            .count(mdoc, date, search)
            .map(|c| c as u32)
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
    use chrono::Utc;
    use rusqlite::Connection;
    use std::sync::Arc;

    // A little wrapper around the real mock that fails on any detail with price == 0,
    // but otherwise delegates to the real MockCustomerTxDetailRepo.
    struct FailingDetailRepo {
        inner: Arc<dyn CustomerTxDetailRepoTrait>,
    }

    impl FailingDetailRepo {
        pub fn new(inner: Arc<dyn CustomerTxDetailRepoTrait>) -> Self {
            Self { inner }
        }
    }

    impl CustomerTxDetailRepoTrait for FailingDetailRepo {
        fn create(&self, d: &CustomerTxDetail) -> Result<(), AppError> {
            if d.price == 0 {
                Err(AppError::Unexpected("detail failure".into()))
            } else {
                self.inner.create(d)
            }
        }

        fn list_by_order(
            &self,
            order_id: i32,
        ) -> Result<Vec<(CustomerTxDetail, String)>, AppError> {
            self.inner.list_by_order(order_id)
        }

        fn create_with_tx(
            &self,
            d: &CustomerTxDetail,
            tx: &rusqlite::Transaction<'_>,
        ) -> Result<(), AppError> {
            if d.price == 0 {
                Err(AppError::Unexpected("detail failure".into()))
            } else {
                self.inner.create_with_tx(d, tx)
            }
        }
    }

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
        Arc<dyn InventoryTransactionRepoTrait>,
        Arc<dyn CustomerTransactionRepoTrait>,
        Arc<dyn CustomerTxDetailRepoTrait>,
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
        let inv_repo: Arc<dyn InventoryTransactionRepoTrait> =
            Arc::new(SqliteInventoryTransactionRepo::new(Arc::clone(&conn)));
        let cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait> =
            Arc::new(SqliteCustomerTransactionRepo::new(Arc::clone(&conn)));
        let cust_tx_detail_repo = Arc::new(SqliteCustomerTxDetailRepo::new(Arc::clone(&conn)));

        let uc = TransactionUseCases::new(
            inv_repo.clone(),
            cust_tx_repo.clone(),
            cust_tx_detail_repo.clone(),
            Arc::clone(&conn),
        );
        (
            uc,
            op_repo,
            prod_repo,
            inv_repo,
            cust_tx_repo,
            cust_tx_detail_repo,
        )
    }

    #[test]
    fn inventory_and_sale_and_stock_flows() -> anyhow::Result<()> {
        let (uc, op_repo, prod_repo, _, _, _) = make_use_cases();

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
        let order_id = uc.sale_transaction(
            CustomerTransaction {
                order_id: 0,
                customer_mdoc: 20,
                operator_mdoc: 10,
                date: None,
                note: None,
            },
            vec![InventoryTransaction {
                operator_mdoc: 10,
                customer_mdoc: Some(20),
                upc: "000000000555".into(),
                quantity_change: -2,
                ..Default::default()
            }],
            vec![CustomerTxDetail {
                detail_id: 0,
                order_id: 0,
                upc: "000000000555".into(),
                quantity: 2,
                price: 1000,
            }],
        )?;
        assert_eq!(order_id, 1);
        let adjusted = uc.list_inv_adjust()?;
        let sale_tx = adjusted
            .iter()
            .find(|tx| tx.customer_mdoc == Some(20) && tx.quantity_change == -2)
            .expect("expected inventory tx with customer_mdoc 20 and quantity_change -2");

        assert_eq!(sale_tx.upc, "000000000555");

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
        let (uc, op_repo, prod_repo, _, _, _) = make_use_cases();

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
        let (uc, op_repo, _prod_repo, _inv, _cust_tx, _det) = make_use_cases();

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

        _prod_repo.create(&Product {
            upc: "00000001".into(),
            desc: "A test product".into(),
            price: 100,
            category: "test-category".into(),
            updated: Some(Utc::now().naive_utc()),
            added: Some(Utc::now().naive_utc()),
            deleted: None,
        })?;

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

    #[test]
    fn sale_transaction_commits_all_repos() -> Result<(), Box<dyn std::error::Error>> {
        let (uc, op_repo, prod_repo, inv, cust_tx, det) = make_use_cases();
        // seed operator and product so we don't violate FKs
        op_repo.create(&Operator {
            mdoc: 1,
            name: "Cashier".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: "A".into(),
            desc: "Sample".into(),
            category: "Test".into(),
            price: 50,
            updated: Some(chrono::Utc::now().naive_utc()),
            added: Some(chrono::Utc::now().naive_utc()),
            deleted: None,
        })?;

        let invs = vec![InventoryTransaction {
            id: None,
            upc: "A".into(),
            quantity_change: 1,
            operator_mdoc: 1,
            customer_mdoc: Some(2),
            ref_order_id: None,
            reference: None,
            created_at: None,
        }];

        let details = vec![CustomerTxDetail {
            detail_id: 0,
            order_id: 0,
            upc: "A".into(),
            quantity: 1,
            price: 50,
        }];

        let ct = CustomerTransaction {
            order_id: 0,
            customer_mdoc: 2,
            operator_mdoc: 1,
            date: None,
            note: None,
        };

        let order_id = uc.sale_transaction(ct, invs, details).unwrap();
        assert!(order_id > 0);
        assert_eq!(inv.list().unwrap().len(), 1);
        assert_eq!(cust_tx.list().unwrap().len(), 1);
        let dets = det.list_by_order(order_id).unwrap();
        assert_eq!(dets.len(), 1);
        Ok(())
    }

    #[test]
    fn sale_transaction_rolls_back_on_detail_error() -> Result<(), AppError> {
        // in-memory DB + full schema with FKs
        let conn = Arc::new(Mutex::new(Connection::open_in_memory()?));
        {
            let db = conn.lock().unwrap();
            db.execute_batch("PRAGMA foreign_keys = ON;")?;
            db.execute_batch(
                r#"
                CREATE TABLE operators (
                    mdoc INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    start TEXT,
                    stop TEXT
                );
                CREATE TABLE products (
                    upc TEXT PRIMARY KEY,
                    desc TEXT NOT NULL,
                    category TEXT NOT NULL,
                    price INTEGER NOT NULL,
                    updated TEXT,
                    added TEXT,
                    deleted TEXT
                );
                CREATE TABLE inventory_transactions (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    upc TEXT NOT NULL REFERENCES products(upc),
                    quantity_change INTEGER NOT NULL,
                    operator_mdoc INTEGER NOT NULL REFERENCES operators(mdoc),
                    customer_mdoc INTEGER REFERENCES operators(mdoc),
                    ref_order_id INTEGER,
                    reference TEXT,
                    created_at TEXT
                );
                CREATE TABLE customer_transactions (
                    order_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    customer_mdoc INTEGER NOT NULL REFERENCES operators(mdoc),
                    operator_mdoc INTEGER NOT NULL REFERENCES operators(mdoc),
                    date TEXT,
                    note TEXT
                );
                CREATE TABLE customer_tx_detail (
                    detail_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    order_id INTEGER NOT NULL REFERENCES customer_transactions(order_id),
                    upc TEXT NOT NULL REFERENCES products(upc),
                    quantity INTEGER NOT NULL,
                    price INTEGER NOT NULL
                );
            "#,
            )?;
        }

        let op_repo: Arc<dyn OperatorRepoTrait> = Arc::new(SqliteOperatorRepo::new(conn.clone()));
        let prod_repo: Arc<dyn ProductRepoTrait> = Arc::new(SqliteProductRepo::new(conn.clone()));
        let inv_repo: Arc<dyn InventoryTransactionRepoTrait> =
            Arc::new(SqliteInventoryTransactionRepo::new(conn.clone()));
        let cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait> =
            Arc::new(SqliteCustomerTransactionRepo::new(conn.clone()));
        let real_det: Arc<dyn CustomerTxDetailRepoTrait> =
            Arc::new(SqliteCustomerTxDetailRepo::new(conn.clone()));
        let fail_det = Arc::new(FailingDetailRepo::new(real_det.clone()));

        // seed FKs
        op_repo.create(&Operator {
            mdoc: 1,
            name: "Cashier".into(),
            start: None,
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: "B".into(),
            desc: "Sample".into(),
            category: "Test".into(),
            price: 100,
            updated: None,
            added: None,
            deleted: None,
        })?;

        // use-case under test
        let uc = TransactionUseCases::new(
            inv_repo.clone(),
            cust_tx_repo.clone(),
            fail_det.clone(),
            conn.clone(),
        );

        // prepare one inventory‐tx + one bad detail (price=0)
        let invs = vec![InventoryTransaction {
            id: None,
            upc: "B".into(),
            quantity_change: 1,
            operator_mdoc: 1,
            customer_mdoc: Some(2),
            ref_order_id: None,
            reference: None,
            created_at: None,
        }];
        let details = vec![CustomerTxDetail {
            detail_id: 0,
            order_id: 0,
            upc: "B".into(),
            quantity: 1,
            price: 0, // triggers FailingDetailRepo
        }];
        let ct = CustomerTransaction {
            order_id: 0,
            customer_mdoc: 2,
            operator_mdoc: 1,
            date: None,
            note: None,
        };

        // run & expect error
        let _err = uc.sale_transaction(ct, invs, details).unwrap_err();

        // assert NO rows committed anywhere
        assert!(
            inv_repo.list_for_today()?.is_empty(),
            "inventory should have rolled back"
        );
        assert!(
            cust_tx_repo.list()?.is_empty(),
            "customer tx should have rolled back"
        );
        assert!(
            real_det.list_by_order(1)?.is_empty(),
            "details should have rolled back"
        );

        Ok(())
    }
}
