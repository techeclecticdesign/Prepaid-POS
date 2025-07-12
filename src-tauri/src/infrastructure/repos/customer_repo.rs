use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::Customer;
use crate::domain::repos::CustomerRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteCustomerRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteCustomerRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl CustomerRepoTrait for SqliteCustomerRepo {
    fn list(&self) -> Result<Vec<Customer>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt =
            conn.prepare("SELECT mdoc, name, added, updated FROM customer ORDER BY added DESC")?;
        let rows = stmt.query_map([], |r| {
            Ok(Customer {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                added: r.get(2)?,
                updated: r.get(3)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Customer>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt =
            conn.prepare("SELECT mdoc, name, added, updated FROM customer WHERE mdoc = ?1")?;
        let mut rows = stmt.query(params![mdoc])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Customer {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                added: r.get(2)?,
                updated: r.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn update(&self, customer: &Customer) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "UPDATE customer SET name = ?1, added = ?2, updated = ?3 WHERE mdoc = ?4",
            params![
                customer.name,
                customer.added,
                customer.updated,
                customer.mdoc
            ],
        )?;
        Ok(())
    }

    fn create(&self, customer: &Customer) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO customer (mdoc, name, added, updated) VALUES (?1, ?2, ?3, ?4)",
            params![
                customer.mdoc,
                customer.name,
                customer.added,
                customer.updated
            ],
        )?;
        Ok(())
    }
}
