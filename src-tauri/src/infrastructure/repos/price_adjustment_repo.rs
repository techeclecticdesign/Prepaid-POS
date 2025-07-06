use crate::common::error::AppError;
use crate::domain::models::PriceAdjustment;
use crate::domain::repos::PriceAdjustmentRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqlitePriceAdjustmentRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqlitePriceAdjustmentRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl PriceAdjustmentRepoTrait for SqlitePriceAdjustmentRepo {
    fn get_by_id(&self, id: i64) -> Result<Option<PriceAdjustment>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, operator_mdoc, upc, old, new, created_at \
         FROM price_adjustments WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(PriceAdjustment {
                id: r.get(0)?,
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
        let conn = self.conn.lock().unwrap();
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
    ) -> Result<(), AppError> {
        tx.execute(
            "INSERT INTO price_adjustments (operator_mdoc, upc, old, new, created_at) \
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
            rusqlite::params![a.operator_mdoc, a.upc, a.old, a.new],
        )?;
        Ok(())
    }

    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<PriceAdjustment>, AppError> {
        let conn = self.conn.lock().unwrap();
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

    fn list_for_product(&self, upc: i64) -> Result<Vec<PriceAdjustment>, AppError> {
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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
}
