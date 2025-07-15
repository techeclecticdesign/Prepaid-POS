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
