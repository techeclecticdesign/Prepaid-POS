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
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl ProductRepoTrait for SqliteProductRepo {
    fn get_by_upc(&self, upc: String) -> Result<Option<Product>, AppError> {
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

    fn get_price(&self, upc: String) -> Result<i32, AppError> {
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

    fn search(
        &self,
        desc_like: Option<String>,
        category: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(Product, i64)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;

        let mut sql = String::from(
            "SELECT p.upc, p.desc, p.category, p.price,
                    p.updated, p.added, p.deleted,
                    COALESCE(inv.available, 0) AS available
             FROM products p
             LEFT JOIN (
               SELECT upc, SUM(quantity_change) AS available
               FROM inventory_transactions
               GROUP BY upc
             ) inv ON p.upc = inv.upc",
        );
        let mut clauses = Vec::new();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut dynamic_params: Vec<String> = Vec::new();

        if let Some(ref s) = desc_like {
            clauses.push("desc LIKE ?");
            let formatted = format!("%{s}%");
            dynamic_params.push(formatted);
            let last = dynamic_params
                .last()
                .ok_or_else(|| AppError::Unexpected("Internal param error".into()))?;
            params.push(last);
        }
        if let Some(ref c) = category {
            clauses.push("category = ?");
            params.push(c);
        }
        clauses.push("deleted IS NULL");
        if !clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }

        sql.push_str(" ORDER BY added DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            // capture both Product and its summed inventory
            let product = Product {
                upc: r.get(0)?,
                desc: r.get(1)?,
                category: r.get(2)?,
                price: r.get(3)?,
                updated: r.get(4)?,
                added: r.get(5)?,
                deleted: r.get(6)?,
            };
            let available: i64 = r.get(7)?;
            Ok((product, available))
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // Returns the total count of products matching the optional filters.
    fn count(&self, desc_like: Option<String>, category: Option<String>) -> Result<u32, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut sql = String::from("SELECT COUNT(*) FROM products");
        let mut clauses = Vec::new();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut dyn_params = Vec::new();

        if let Some(ref s) = desc_like {
            clauses.push("desc LIKE ?");
            dyn_params.push(format!("%{s}%"));
            let last = dyn_params
                .last()
                .ok_or_else(|| AppError::Unexpected("Internal param error".into()))?;
            params.push(last);
        }
        if let Some(ref c) = category {
            clauses.push("category = ?");
            params.push(c);
        }
        if !clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }

        let count: u32 = conn.query_row(&sql, params.as_slice(), |r| r.get(0))?;
        Ok(count)
    }
}
