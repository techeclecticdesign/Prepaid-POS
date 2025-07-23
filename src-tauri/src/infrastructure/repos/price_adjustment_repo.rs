use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::PriceAdjustment;
use crate::domain::repos::PriceAdjustmentRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqlitePriceAdjustmentRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqlitePriceAdjustmentRepo {
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl PriceAdjustmentRepoTrait for SqlitePriceAdjustmentRepo {
    fn get_by_id(&self, id: i32) -> Result<Option<PriceAdjustment>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, operator_mdoc, upc, old, new, created_at \
         FROM price_adjustments WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(PriceAdjustment {
                id: r.get::<_, i32>(0)?,
                operator_mdoc: r.get(1)?,
                upc: r.get(2)?,
                old: r.get(3)?,
                new: r.get(4)?,
                created_at: r.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, a: &PriceAdjustment) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO price_adjustments (operator_mdoc, upc, old, new) VALUES (?1, ?2, ?3, ?4)",
            params![a.operator_mdoc, a.upc, a.old, a.new],
        )?;
        Ok(())
    }

    fn create_with_tx(
        &self,
        a: &PriceAdjustment,
        tx: &rusqlite::Transaction<'_>,
    ) -> Result<i32, AppError> {
        tx.execute(
            "INSERT INTO price_adjustments (operator_mdoc, upc, old, new, created_at) \
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
            rusqlite::params![a.operator_mdoc, a.upc, a.old, a.new],
        )?;
        Ok(tx.last_insert_rowid() as i32)
    }

    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<PriceAdjustment>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt =
            conn.prepare(
              "SELECT id, operator_mdoc, upc, old, new, created_at FROM price_adjustments WHERE operator_mdoc = ?1"
            )?;
        let adjustments = stmt
            .query_map(params![operator_mdoc], |r| {
                Ok(PriceAdjustment {
                    id: r.get(0)?,
                    operator_mdoc: r.get(1)?,
                    upc: r.get(2)?,
                    old: r.get(3)?,
                    new: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(adjustments)
    }

    fn list_for_product(&self, upc: String) -> Result<Vec<PriceAdjustment>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, operator_mdoc, upc, old, new, created_at FROM price_adjustments WHERE upc = ?1"
        )?;
        let items = stmt
            .query_map(params![upc], |r| {
                Ok(PriceAdjustment {
                    id: r.get(0)?,
                    operator_mdoc: r.get(1)?,
                    upc: r.get(2)?,
                    old: r.get(3)?,
                    new: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    fn list_for_today(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, operator_mdoc, upc, old, new, created_at FROM price_adjustments WHERE date(created_at) = date('now')"
        )?;
        let items = stmt
            .query_map([], |r| {
                Ok(PriceAdjustment {
                    id: r.get(0)?,
                    operator_mdoc: r.get(1)?,
                    upc: r.get(2)?,
                    old: r.get(3)?,
                    new: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    fn list(&self) -> Result<Vec<PriceAdjustment>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, operator_mdoc, upc, old, new, created_at FROM price_adjustments",
        )?;
        let items = stmt
            .query_map([], |r| {
                Ok(PriceAdjustment {
                    id: r.get(0)?,
                    operator_mdoc: r.get(1)?,
                    upc: r.get(2)?,
                    old: r.get(3)?,
                    new: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(PriceAdjustment, String, String)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT pa.id,
                   pa.operator_mdoc,
                   pa.upc,
                   pa.old,
                   pa.new,
                   pa.created_at,
                   p.desc AS product_name,
                   o.name AS operator_name
            FROM price_adjustments pa
            JOIN products  p ON pa.upc           = p.upc
            JOIN operators o ON pa.operator_mdoc = o.mdoc
            WHERE 1=1
        "
        .to_string();

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        // date filter
        if let Some(ref d) = date {
            sql.push_str(" AND date(pa.created_at)=date(?)");
            params.push(d);
        }
        // text search on upc/operator_mdoc
        if let Some(ref s) = search {
            sql.push_str(
                " AND (pa.upc LIKE ? \
                          OR o.name LIKE ? \
                          OR p.desc LIKE ? \
                          OR pa.operator_mdoc LIKE ?)",
            );
            let pat = format!("%{s}%");
            string_params.push(pat);
            let last = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("search pattern missing".to_string()))?;
            for _ in 0..4 {
                params.push(last);
            }
        }

        sql.push_str(" ORDER BY pa.created_at DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            let pa = PriceAdjustment {
                id: r.get(0)?,
                operator_mdoc: r.get(1)?,
                upc: r.get(2)?,
                old: r.get(3)?,
                new: r.get(4)?,
                created_at: r.get(5)?,
            };
            let product_name: String = r.get(6)?;
            let operator_name: String = r.get(7)?;
            Ok((pa, product_name, operator_name))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT COUNT(*) \
            FROM price_adjustments pa \
            JOIN products  p ON pa.upc           = p.upc \
            JOIN operators o ON pa.operator_mdoc = o.mdoc \
            WHERE 1=1"
            .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();
        if let Some(ref d) = date {
            sql.push_str(" AND date(pa.created_at)=date(?)");
            params.push(d);
        }
        if let Some(ref s) = search {
            sql.push_str(
                " AND (pa.upc LIKE ? \
                          OR o.name LIKE ? \
                          OR p.desc LIKE ? \
                          OR pa.operator_mdoc LIKE ?)",
            );
            let pat = format!("%{s}%");
            string_params.push(pat);
            let last = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("search pattern missing".to_string()))?;
            for _ in 0..4 {
                params.push(last);
            }
        }
        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| r.get(0))
            .map_err(Into::into)
    }
}
