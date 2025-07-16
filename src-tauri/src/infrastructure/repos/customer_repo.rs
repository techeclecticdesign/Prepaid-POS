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

    fn search(
        &self,
        limit: i64,
        offset: i64,
        search: Option<String>,
    ) -> Result<Vec<Customer>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT mdoc, name, added, updated \
            FROM customer WHERE 1=1\
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        if let Some(ref s) = search {
            sql.push_str(" AND (mdoc LIKE ? OR name LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let p = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("customer search pattern missing".into()))?;
            params.push(p);
            params.push(p);
        }

        sql.push_str(" ORDER BY added DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            Ok(Customer {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                added: r.get(2)?,
                updated: r.get(3)?,
            })
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, search: Option<String>) -> Result<i64, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "SELECT COUNT(*) FROM customer WHERE 1=1".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        if let Some(ref s) = search {
            sql.push_str(" AND (mdoc LIKE ? OR name LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let p = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("customer count pattern missing".into()))?;
            params.push(p);
            params.push(p);
        }

        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| r.get(0))
            .map_err(Into::into)
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
