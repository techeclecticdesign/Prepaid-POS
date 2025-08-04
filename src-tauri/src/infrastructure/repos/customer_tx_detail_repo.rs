use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::customer_tx_detail::CustomerTxDetail;
use crate::domain::report_models::product_sales::{ProductSalesByCategory, SalesTotals};
use crate::domain::repos::customer_tx_detail_repo_trait::CustomerTxDetailRepoTrait;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteCustomerTxDetailRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCustomerTxDetailRepo {
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
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

    // for use with atomic_tx
    fn create_with_tx(
        &self,
        d: &CustomerTxDetail,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError> {
        if d.detail_id > 0 {
            tx.execute(
                "INSERT INTO customer_tx_detail
                 (detail_id, order_id, upc, quantity, price)
                 VALUES (?1,?2,?3,?4,?5)",
                rusqlite::params![d.detail_id, d.order_id, d.upc, d.quantity, d.price],
            )?;
        } else {
            tx.execute(
                "INSERT INTO customer_tx_detail
                 (order_id, upc, quantity, price)
                 VALUES (?1,?2,?3,?4)",
                rusqlite::params![d.order_id, d.upc, d.quantity, d.price],
            )?;
        }
        let detail_id = tx.last_insert_rowid() as i32;
        Ok(detail_id)
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
            let raw_detail_id: i64 = r.get(0)?;
            let raw_order_id: i64 = r.get(1)?;
            let detail = CustomerTxDetail {
                detail_id: raw_detail_id as i32,
                order_id: raw_order_id as i32,
                upc: r.get(2)?,
                quantity: r.get(4)?,
                price: r.get(5)?,
            };
            let product_name: String = r.get(3)?;
            Ok((detail, product_name))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn sales_by_category(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<ProductSalesByCategory>, AppError> {
        let conn = self.conn.safe_lock()?;
        // build detail, per‐category summary, and grand total CTEs, then union them
        let sql = r#"
        WITH detail AS (
          SELECT
            p.category,
            d.upc,
            p.desc    AS name,
            d.price,
            SUM(d.quantity)           AS quantity_sold,
            SUM(d.quantity * d.price) AS total_sales,
            0                          AS is_summary
          FROM customer_tx_detail d
          JOIN customer_transactions t
            ON d.order_id = t.order_id
           AND t.date >= ?1
           AND t.date <  ?2
          JOIN products p
            ON d.upc = p.upc
          GROUP BY p.category, d.upc
        ), category_total AS (
          SELECT
            category,
            ''      AS upc,
            'Category Total' AS name,
            0      AS price,
            SUM(quantity_sold)   AS quantity_sold,
            SUM(total_sales)     AS total_sales,
            1             AS is_summary
          FROM detail
          GROUP BY category
        ), grand_total AS (
          SELECT
            ''            AS category,
            ''            AS upc,
            'Grand Total'   AS name,
            0            AS price,
            SUM(quantity_sold)   AS quantity_sold,
            SUM(total_sales)     AS total_sales,
            2             AS is_summary
          FROM detail
        )
        SELECT *      FROM detail
        UNION ALL
        SELECT *      FROM category_total
        UNION ALL
        SELECT *      FROM grand_total
        ORDER BY 1, 7, 3
        "#;

        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![start, end], |r| {
            Ok(ProductSalesByCategory {
                category: r.get("category")?,
                upc: r.get("upc")?,
                name: r.get("name")?,
                price: r.get::<_, i64>("price")? as i32,
                quantity_sold: r.get::<_, i64>("quantity_sold")? as i32,
                total_sales: r.get::<_, i64>("total_sales")? as i32,
                is_summary: r.get::<_, i64>("is_summary")? != 0,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    fn get_sales_totals(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<SalesTotals, AppError> {
        let conn = self.conn.safe_lock()?;
        let sql = r#"
            SELECT
              COALESCE(SUM(d.quantity), 0)             AS total_quantity,
              COALESCE(SUM(d.quantity * d.price), 0)   AS total_value
            FROM customer_tx_detail d
            JOIN customer_transactions t
              ON d.order_id = t.order_id
             AND t.date >= ?1
             AND t.date <  ?2
        "#;

        let row = conn.query_row(sql, params![start, end], |r| {
            Ok(SalesTotals {
                total_quantity: r.get::<_, i64>(0)? as i32,
                total_value: r.get::<_, i64>(1)? as i32,
            })
        })?;

        Ok(row)
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
