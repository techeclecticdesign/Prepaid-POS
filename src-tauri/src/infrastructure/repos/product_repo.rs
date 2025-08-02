use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::Product;
use crate::domain::report_models::product_inventory::ProductInventoryReport;
use crate::domain::report_models::product_inventory::ProductInventoryTotals;
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
        // filter: nonzero price, not deleted; sort by category then name
        let mut stmt = conn.prepare(
            "SELECT upc, desc, category, price, updated, added, deleted
            FROM products
            WHERE price != 0 AND deleted IS NULL
            ORDER BY category, desc",
        )?;
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
        limit: i32,
        offset: i32,
    ) -> Result<Vec<(Product, i32)>, AppError> {
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

        sql.push_str(" ORDER BY desc ASC LIMIT ? OFFSET ?");
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
            Ok((product, available as i32))
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // Returns the total count of products matching the optional filters.
    fn count(&self, desc_like: Option<String>, category: Option<String>) -> Result<i32, AppError> {
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

        let count: i32 = conn.query_row(&sql, params.as_slice(), |r| r.get(0))?;
        Ok(count)
    }

    // to keep architecture thin we use CTE to do 2 queries, and union them into a combined result
    fn report_by_category(&self) -> Result<Vec<ProductInventoryReport>, AppError> {
        let conn = self.conn.safe_lock()?;
        let sql = r#"
            WITH detail AS (
                SELECT
                  p.category,
                  p.upc,
                  p.desc    AS name,
                  p.price,
                  COALESCE(SUM(it.quantity_change), 0) AS quantity,
                  COALESCE(SUM(it.quantity_change), 0) * p.price AS total
                FROM products p
                LEFT JOIN inventory_transactions it
                  ON p.upc = it.upc
                GROUP BY p.category, p.upc
                HAVING quantity != 0
            ), summary AS (
                SELECT
                  category,
                  NULL    AS upc,
                  NULL    AS name,
                  NULL    AS price,
                  SUM(quantity) AS quantity,
                  SUM(total)    AS total
                FROM detail
                GROUP BY category
                HAVING SUM(quantity) != 0
            )
            SELECT *, 0 AS is_summary FROM detail
            UNION ALL
            SELECT *, 1 AS is_summary FROM summary
            ORDER BY category, is_summary, name;
        "#;
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |r| {
            Ok(ProductInventoryReport {
                category: r.get("category")?,
                upc: r.get("upc")?,
                name: r.get("name")?,
                price: r.get("price")?,
                quantity: r.get("quantity")?,
                total: r.get("total")?,
                is_summary: r.get::<_, i32>("is_summary")? != 0,
            })
        })?;
        let rows = rows.collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    fn get_inventory_totals(&self) -> Result<ProductInventoryTotals, AppError> {
        let conn = self.conn.safe_lock()?;
        let sql = r#"
            SELECT
              SUM(quantity) AS total_quantity,
              SUM(total)    AS total_value
            FROM (
              SELECT
                COALESCE(SUM(it.quantity_change), 0) AS quantity,
                COALESCE(SUM(it.quantity_change), 0) * p.price AS total
              FROM products p
              LEFT JOIN inventory_transactions it ON p.upc = it.upc
              GROUP BY p.upc
              HAVING quantity != 0
            )
        "#;

        let row = conn.query_row(sql, [], |r| {
            Ok(ProductInventoryTotals {
                total_quantity: r.get::<_, i32>(0)?,
                total_value: r.get::<_, i32>(1)?,
            })
        })?;
        Ok(row)
    }
}
