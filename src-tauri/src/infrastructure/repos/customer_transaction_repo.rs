use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::CustomerTransaction;
use crate::domain::repos::CustomerTransactionRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteCustomerTransactionRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCustomerTransactionRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl CustomerTransactionRepoTrait for SqliteCustomerTransactionRepo {
    fn create(&self, tx: &CustomerTransaction) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        if tx.order_id > 0 {
            conn.execute(
                "INSERT INTO customer_transactions
                 (order_id, customer_mdoc, operator_mdoc, date, note)
                 VALUES (?1,?2,?3,?4,?5)",
                params![
                    tx.order_id,
                    tx.customer_mdoc,
                    tx.operator_mdoc,
                    tx.date,
                    tx.note,
                ],
            )?;
        } else {
            conn.execute(
                "INSERT INTO customer_transactions
                 (customer_mdoc, operator_mdoc, date, note)
                 VALUES (?1,?2,?3,?4)",
                params![tx.customer_mdoc, tx.operator_mdoc, tx.date, tx.note,],
            )?;
        }
        Ok(())
    }

    fn get(&self, order_id: i32) -> Result<Option<CustomerTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT order_id, customer_mdoc, operator_mdoc, date, note
             FROM customer_transactions WHERE order_id = ?1",
        )?;
        let mut rows = stmt.query(params![order_id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(CustomerTransaction {
                order_id: r.get(0)?,
                customer_mdoc: r.get(1)?,
                operator_mdoc: r.get(2)?,
                date: r.get(3)?,
                note: r.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn list(&self) -> Result<Vec<CustomerTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT order_id, customer_mdoc, operator_mdoc, date, note
             FROM customer_transactions",
        )?;
        let txs = stmt
            .query_map([], |r| {
                Ok(CustomerTransaction {
                    order_id: r.get(0)?,
                    customer_mdoc: r.get(1)?,
                    operator_mdoc: r.get(2)?,
                    date: r.get(3)?,
                    note: r.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(txs)
    }

    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<CustomerTransaction>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT order_id, customer_mdoc, operator_mdoc, date, note \
            FROM customer_transaction WHERE 1=1\
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        // date filter
        if let Some(ref d) = date {
            sql.push_str(" AND date(date)=date(?)");
            params.push(d);
        }
        // search on multiple fields
        if let Some(ref s) = search {
            sql.push_str(" AND (customer_mdoc LIKE ? OR operator_mdoc LIKE ? OR order_id LIKE ? OR note LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let p = string_params.last().ok_or_else(|| {
                AppError::Unexpected("customer_transaction pattern missing".into())
            })?;
            // push four times for each placeholder
            params.push(p);
            params.push(p);
            params.push(p);
            params.push(p);
        }

        // ordering + pagination
        sql.push_str(" ORDER BY date DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            Ok(CustomerTransaction {
                order_id: r.get(0)?,
                customer_mdoc: r.get(1)?,
                operator_mdoc: r.get(2)?,
                date: r.get(3)?,
                note: r.get(4)?,
            })
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "SELECT COUNT(*) FROM customer_transaction WHERE 1=1".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        if let Some(ref d) = date {
            sql.push_str(" AND date(date)=date(?)");
            params.push(d);
        }
        if let Some(ref s) = search {
            sql.push_str(" AND (customer_mdoc LIKE ? OR operator_mdoc LIKE ? OR order_id LIKE ? OR note LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let p = string_params.last().ok_or_else(|| {
                AppError::Unexpected("customer_transaction count pattern missing".into())
            })?;
            params.push(p);
            params.push(p);
            params.push(p);
            params.push(p);
        }

        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| r.get(0))
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod repo_tests {
    use super::*;
    use crate::infrastructure::db::create_connection;
    use std::sync::Arc;

    impl SqliteCustomerTransactionRepo {
        pub fn create_table_if_not_exists(&self) -> Result<(), AppError> {
            let conn = self.conn.safe_lock()?;
            conn.execute_batch(
                "
            CREATE TABLE IF NOT EXISTS customer_transactions (
                order_id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_mdoc INTEGER NOT NULL,
                operator_mdoc INTEGER NOT NULL,
                date TEXT,
                note TEXT
            );
            ",
            )?;
            Ok(())
        }
    }

    #[test]
    fn repo_crud_flow() {
        let conn = Arc::new(create_connection(":memory:").unwrap());
        let repo = SqliteCustomerTransactionRepo::new(Arc::clone(&conn));

        repo.create_table_if_not_exists().unwrap();

        assert!(repo.get(1).unwrap().is_none());
        assert!(repo.list().unwrap().is_empty());

        // insert with auto-id
        let tx = CustomerTransaction {
            order_id: 0,
            customer_mdoc: 3,
            operator_mdoc: 4,
            date: None,
            note: Some("hi".into()),
        };
        repo.create(&tx).unwrap();
        let all = repo.list().unwrap();
        assert_eq!(all.len(), 1);
        let id = all[0].order_id;
        assert!(id > 0);

        // can get by that id
        let got = repo.get(id).unwrap().unwrap();
        assert_eq!(got.customer_mdoc, 3);

        // explicit-id insert
        let tx2 = CustomerTransaction {
            order_id: 7,
            ..tx.clone()
        };
        repo.create(&tx2).unwrap();
        assert!(repo.get(7).unwrap().is_some());
    }
}
