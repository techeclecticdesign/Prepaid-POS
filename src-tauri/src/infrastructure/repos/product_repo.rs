use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::Product;
use crate::domain::repos::ProductRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteProductRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteProductRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl ProductRepoTrait for SqliteProductRepo {
    fn get_by_upc(&self, upc: i64) -> Result<Option<Product>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT upc, desc, category, price, updated, added, deleted 
             FROM products WHERE upc = ?1",
        )?;
        let mut rows = stmt.query(params![upc])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Product {
                upc: r.get(0)?,
                desc: r.get(1)?,
                category: r.get(2)?,
                price: r.get(3)?,
                updated: r.get(4)?,
                added: r.get(5)?,
                deleted: r.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_price(&self, upc: i64) -> Result<i32, AppError> {
        let conn = self.conn.safe_lock()?;
        let price: i32 = conn.query_row(
            "SELECT price FROM products WHERE upc = ?1",
            params![upc],
            |r| r.get(0),
        )?;
        Ok(price)
    }

    fn create(&self, p: &Product) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO products (upc, desc, category, price, updated, added, deleted) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![p.upc, p.desc, p.category, p.price, p.updated, p.added, p.deleted],
        )?;
        Ok(())
    }

    fn update_by_upc(&self, p: &Product) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "UPDATE products SET desc = ?1, category = ?2, price = ?3, 
             updated = ?4, deleted = ?5 WHERE upc = ?6",
            params![p.desc, p.category, p.price, p.updated, p.deleted, p.upc],
        )?;
        Ok(())
    }

    fn update_by_upc_with_tx(
        &self,
        p: &Product,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<(), AppError> {
        tx.execute(
            "UPDATE products SET desc = ?1, category = ?2, price = ?3, \
         updated = ?4, deleted = ?5 WHERE upc = ?6",
            rusqlite::params![p.desc, p.category, p.price, p.updated, p.deleted, p.upc],
        )?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Product>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn
            .prepare("SELECT upc, desc, category, price, updated, added, deleted FROM products")?;
        let prods = stmt
            .query_map([], |r| {
                Ok(Product {
                    upc: r.get(0)?,
                    desc: r.get(1)?,
                    category: r.get(2)?,
                    price: r.get(3)?,
                    updated: r.get(4)?,
                    added: r.get(5)?,
                    deleted: r.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(prods)
    }
}
