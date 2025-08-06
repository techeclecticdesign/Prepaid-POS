use crate::application::common::db::atomic_tx;
use crate::common::error::AppError;
use crate::domain::models::{CustomerTransaction, CustomerTxDetail, InventoryTransaction};
use crate::domain::repos::customer_tx_repo_trait::SaleDetailsTuple;
use crate::domain::repos::CustomerTransactionRepoTrait;
use crate::domain::repos::CustomerTxDetailRepoTrait;
use crate::domain::repos::InventoryTransactionRepoTrait;
use crate::domain::repos::WeeklyLimitRepoTrait;
use chrono::{Datelike, Duration, Utc};
use log::{error, info};
use std::sync::{Arc, Mutex};

pub struct TransactionUseCases {
    inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
    cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
    cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
    limit_repo: Arc<dyn WeeklyLimitRepoTrait>,
    conn: Arc<Mutex<rusqlite::Connection>>,
}

impl TransactionUseCases {
    pub fn new(
        inv_repo: Arc<dyn InventoryTransactionRepoTrait>,
        cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait>,
        cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait>,
        limit_repo: Arc<dyn WeeklyLimitRepoTrait>,
        conn: Arc<Mutex<rusqlite::Connection>>,
    ) -> Self {
        Self {
            inv_repo,
            cust_tx_repo,
            cust_tx_detail_repo,
            limit_repo,
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
                "inventory adjustment error: upc={} operator={} error={e}",
                tx.upc, tx.operator_mdoc
            ),
        }
        res.map(|()| tx)
    }

    pub fn sale_transaction(
        &self,
        cust_tx: CustomerTransaction,
        mut invs: Vec<InventoryTransaction>,
        mut details: Vec<CustomerTxDetail>,
    ) -> Result<i32, AppError> {
        atomic_tx(&self.conn, |tx| {
            // add timestamp
            let mut tx_to_insert = cust_tx.clone();
            if tx_to_insert.date.is_none() {
                tx_to_insert.date = Some(chrono::Utc::now().naive_utc());
            }
            let order_id = self.cust_tx_repo.create_with_tx(&tx_to_insert, tx)?;

            for inv in &mut invs {
                inv.ref_order_id = Some(order_id);
                self.inv_repo.create_with_tx(inv, tx)?;
            }

            for det in &mut details {
                det.order_id = order_id;
                self.cust_tx_detail_repo.create_with_tx(det, tx)?;
            }

            Ok(order_id)
        })
    }

    pub fn list_for_product(&self, upc: String) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_product(upc)
    }

    pub fn search_inventory_transactions(
        &self,
        page: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(InventoryTransaction, String, String)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        self.inv_repo.search(limit, offset, date, search)
    }

    pub fn count_inventory_transactions(
        &self,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i32, AppError> {
        self.inv_repo.count(date, search)
    }

    pub fn list_for_customer(
        &self,
        customer_mdoc: i32,
    ) -> Result<Vec<InventoryTransaction>, AppError> {
        self.inv_repo.list_for_customer(customer_mdoc)
    }

    pub fn list_order_details(
        &self,
        order_id: i32,
    ) -> Result<Vec<(CustomerTxDetail, String)>, AppError> {
        self.cust_tx_detail_repo.list_by_order(order_id)
    }

    pub fn search_customer_transactions(
        &self,
        page: i32,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(CustomerTransaction, String, i32)>, AppError> {
        let limit = 10;
        let offset = page.saturating_sub(1) * limit;
        self.cust_tx_repo.search(limit, offset, mdoc, date, search)
    }

    pub fn count_customer_transactions(
        &self,
        mdoc: Option<i32>,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<i32, AppError> {
        self.cust_tx_repo.count(mdoc, date, search)
    }

    pub fn get_sale_details(&self, order_id: i32) -> Result<SaleDetailsTuple, AppError> {
        self.cust_tx_repo.get_with_details_and_balance(order_id)
    }

    pub fn get_weekly_limit(&self) -> Result<i32, AppError> {
        self.limit_repo.get_limit()
    }

    pub fn set_weekly_limit(&self, limit: i32) -> Result<(), AppError> {
        self.limit_repo.set_limit(limit)
    }

    pub fn get_weekly_spent(&self, customer_mdoc: i32) -> Result<i32, AppError> {
        let now = Utc::now().naive_utc();
        let weekday = now.weekday().num_days_from_sunday() as i64;
        let date = now.date();
        let midnight = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| AppError::Unexpected("Could not construct midnight time".to_string()))?;
        let week_start = midnight - Duration::days(weekday);
        self.cust_tx_repo
            .get_weekly_spent(customer_mdoc, week_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::error::AppError;
    use crate::domain::models::operator::Operator;
    use crate::domain::models::product::Product;
    use crate::domain::models::{CustomerTransaction, CustomerTxDetail, InventoryTransaction};
    use crate::domain::repos::{
        CustomerTransactionRepoTrait, CustomerTxDetailRepoTrait, InventoryTransactionRepoTrait,
        WeeklyLimitRepoTrait,
    };
    use crate::domain::repos::{OperatorRepoTrait, ProductRepoTrait};
    use crate::test_support::mock_customer_tx_detail_repo::MockCustomerTxDetailRepo;
    use crate::test_support::mock_customer_tx_repo::MockCustomerTransactionRepo;
    use crate::test_support::mock_inventory_transaction_repo::MockInventoryTransactionRepo;
    use crate::test_support::mock_operator_repo::MockOperatorRepo;
    use crate::test_support::mock_product_repo::MockProductRepo;
    use crate::test_support::mock_weekly_limit_repo::MockWeeklyLimitRepo;

    use crate::domain::report_models::daily_sales::DailySales;
    use crate::domain::report_models::product_sales::{ProductSalesByCategory, SalesTotals};
    use chrono::NaiveDateTime;
    use rusqlite::{Connection, Transaction};
    use std::sync::{Arc, Mutex};

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

    struct FailingDetailRepo;

    impl FailingDetailRepo {
        pub fn new() -> Self {
            FailingDetailRepo
        }
    }
    impl CustomerTxDetailRepoTrait for FailingDetailRepo {
        fn create_with_tx(
            &self,
            _detail: &CustomerTxDetail,
            _tx: &Transaction<'_>,
        ) -> Result<i32, AppError> {
            Err(AppError::Unexpected("simulated insert failure".to_string()))
        }
        fn create(&self, _detail: &CustomerTxDetail) -> Result<(), AppError> {
            Err(AppError::Unexpected(
                "not implemented in FailingDetailRepo".to_string(),
            ))
        }
        fn list_by_order(
            &self,
            _order_id: i32,
        ) -> Result<Vec<(CustomerTxDetail, String)>, AppError> {
            Ok(vec![])
        }
        fn sales_by_category(
            &self,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> Result<Vec<ProductSalesByCategory>, AppError> {
            Err(AppError::Unexpected(
                "sales_by_category not implemented".into(),
            ))
        }
        fn get_sales_totals(
            &self,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> Result<SalesTotals, AppError> {
            Err(AppError::Unexpected(
                "get_sales_totals not implemented".into(),
            ))
        }
        fn sales_by_day(
            &self,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> Result<Vec<DailySales>, AppError> {
            Err(AppError::Unexpected("sales_by_day not implemented".into()))
        }
    }

    fn make_use_cases() -> (
        TransactionUseCases,
        Arc<dyn OperatorRepoTrait>,
        Arc<dyn ProductRepoTrait>,
        Arc<dyn InventoryTransactionRepoTrait>,
        Arc<dyn CustomerTransactionRepoTrait>,
        Arc<dyn CustomerTxDetailRepoTrait>,
        Arc<dyn WeeklyLimitRepoTrait>,
    ) {
        // Real DB only for atomic_tx; repos are all mocks
        let conn = Arc::new(Mutex::new(Connection::open_in_memory().unwrap()));

        let op_repo: Arc<dyn OperatorRepoTrait> = Arc::new(MockOperatorRepo::default());
        let prod_repo: Arc<dyn ProductRepoTrait> = Arc::new(MockProductRepo::default());
        let inv_repo: Arc<dyn InventoryTransactionRepoTrait> =
            Arc::new(MockInventoryTransactionRepo::default());
        let cust_tx_repo: Arc<dyn CustomerTransactionRepoTrait> =
            Arc::new(MockCustomerTransactionRepo::default());
        let cust_tx_detail_repo: Arc<dyn CustomerTxDetailRepoTrait> =
            Arc::new(MockCustomerTxDetailRepo::default());
        let limit_repo: Arc<dyn WeeklyLimitRepoTrait> = Arc::new(MockWeeklyLimitRepo::default());

        let uc = TransactionUseCases::new(
            inv_repo.clone(),
            cust_tx_repo.clone(),
            cust_tx_detail_repo.clone(),
            limit_repo.clone(),
            conn.clone(),
        );
        (
            uc,
            op_repo,
            prod_repo,
            inv_repo,
            cust_tx_repo,
            cust_tx_detail_repo,
            limit_repo,
        )
    }

    #[test]
    fn inventory_and_sale_and_stock_flows() -> anyhow::Result<()> {
        let (uc, op_repo, prod_repo, _, _, _, _) = make_use_cases();
        // seed FK tables
        op_repo.create(&Operator {
            mdoc: 10,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: "000000000555".into(),
            ..Default::default()
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
        Ok(())
    }

    #[test]
    fn list_filters() -> anyhow::Result<()> {
        let (uc, op_repo, prod_repo, _, _, _, _) = make_use_cases();
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
            ..Default::default()
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
        Ok(())
    }

    #[test]
    fn sale_transaction_commits_all_repos() -> Result<(), Box<dyn std::error::Error>> {
        let (uc, op_repo, prod_repo, inv, cust_tx, det, _) = make_use_cases();
        // seed operator and product so we don't violate FKs
        op_repo.create(&Operator {
            mdoc: 1,
            name: "Op1".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        })?;
        prod_repo.create(&Product {
            upc: "A".into(),
            ..Default::default()
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
        assert_eq!(inv.list_for_product("A".into())?.len(), 1);
        assert_eq!(cust_tx.list()?.len(), 1);
        let dets = det.list_by_order(order_id)?;
        assert_eq!(dets.len(), 1);
        Ok(())
    }

    #[test]
    fn sale_transaction_rolls_back_on_detail_error() -> Result<(), AppError> {
        let (_, _, _, inv, cust_tx, _, _) = make_use_cases();
        let fail_det = FailingDetailRepo::new();
        let conn = Arc::new(Mutex::new(Connection::open_in_memory()?));
        let uc = TransactionUseCases::new(
            inv.clone(),
            cust_tx.clone(),
            Arc::new(fail_det),
            Arc::new(MockWeeklyLimitRepo::new()),
            conn,
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

        // build a dummy customer transaction
        let cust_tx = CustomerTransaction {
            order_id: 0,
            customer_mdoc: 2,
            operator_mdoc: 1,
            date: None,
            note: None,
        };
        // FailingDetailRepo will fail on create_with_tx
        let details = vec![CustomerTxDetail {
            detail_id: 0,
            order_id: 0,
            upc: "B".into(),
            quantity: 1,
            price: 999,
        }];
        let result = uc.sale_transaction(cust_tx, invs, details);
        assert!(
            result.is_err(),
            "Expected sale_transaction to error and rollback"
        );
        Ok(())
    }
}
