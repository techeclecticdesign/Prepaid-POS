use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::repos::customer_tx_detail_repo_trait::CustomerTxDetailRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteCustomerTxDetailRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCustomerTxDetailRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl CustomerTxDetailRepoTrait for SqliteCustomerTxDetailRepo {
    fn create(&self, d: &CustomerTxDetail) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        if d.detail_id > 0 {
            conn.execute(
                "INSERT INTO customer_tx_detail
                 (detail_id, order_id, upc, quantity, price)
                 VALUES (?1,?2,?3,?4,?5)",
                params![d.detail_id, d.order_id, d.upc, d.quantity, d.price],
            )?;
        } else {
            conn.execute(
                "INSERT INTO customer_tx_detail
                 (order_id, upc, quantity, price)
                 VALUES (?1,?2,?3,?4)",
                params![d.order_id, d.upc, d.quantity, d.price],
            )?;
        }
        Ok(())
    }

    fn list_by_order(&self, order_id: i32) -> Result<Vec<(CustomerTxDetail, String)>, AppError> {
        let conn = self.conn.safe_lock()?;
        // join to grab product_name
        let mut stmt = conn.prepare(
            "SELECT d.detail_id,
                    d.order_id,
                    d.upc,
                    p.desc      AS product_name,
                    d.quantity,
                    d.price
             FROM customer_tx_detail d
             JOIN products p ON d.upc = p.upc
             WHERE d.order_id = ?1",
        )?;
        let rows = stmt.query_map(params![order_id], |r| {
            let detail = CustomerTxDetail {
                detail_id: r.get(0)?,
                order_id: r.get(1)?,
                upc: r.get(2)?,
                quantity: r.get(4)?,
                price: r.get(5)?,
            };
            let product_name: String = r.get(3)?;
            Ok((detail, product_name))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }
}

#[cfg(test)]
mod repo_tests {
    use super::*;
    use crate::domain::models::customer_tx_detail::CustomerTxDetail;
    use crate::infrastructure::db::create_connection;
    use std::sync::Arc;

    fn make_repo() -> SqliteCustomerTxDetailRepo {
        let mtx_conn = create_connection(":memory:").unwrap();
        {
            let conn = mtx_conn.lock().unwrap();
            conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
        }
        // now wrap it in an Arc so it matches the repo constructor
        let arc = Arc::new(mtx_conn);
        {
            let conn = arc.lock().unwrap();
            conn.execute_batch(
                "
            DELETE FROM customer_transactions;
            DELETE FROM customer_tx_detail;
            INSERT INTO customer_transactions (order_id, customer_mdoc, operator_mdoc, date)
            VALUES (100, 1, 1, CURRENT_TIMESTAMP);
            CREATE TABLE IF NOT EXISTS products (
            upc      TEXT PRIMARY KEY,
            desc     TEXT NOT NULL,
            category TEXT NOT NULL,
            price    INTEGER NOT NULL,
            updated  DATETIME NOT NULL,
            added    DATETIME NOT NULL,
            deleted  DATETIME
            );
            INSERT INTO products (upc, desc, category, price, updated, added)
              VALUES ('00000001', '', '', 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            ",
            )
            .unwrap();
        }
        SqliteCustomerTxDetailRepo::new(arc)
    }

    #[test]
    fn repo_crud_flow() {
        let repo = make_repo();
        assert!(repo.list_by_order(100).unwrap().is_empty());

        let d1 = CustomerTxDetail {
            detail_id: 0,
            order_id: 100,
            upc: "00000001".into(),
            quantity: 2,
            price: 150,
        };
        repo.create(&d1).unwrap();

        let list = repo.list_by_order(100).unwrap();
        assert_eq!(list.len(), 1);
        let (d, _) = &list[0];
        assert_eq!(d.detail_id, 1);

        let d2 = CustomerTxDetail {
            detail_id: 7,
            ..d1.clone()
        };
        repo.create(&d2).unwrap();
        let list2 = repo.list_by_order(100).unwrap();
        assert!(list2.iter().any(|(d, _)| d.detail_id == 7));
    }
}
