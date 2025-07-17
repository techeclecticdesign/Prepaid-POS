use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::InventoryTransaction;
use crate::domain::repos::InventoryTransactionRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteInventoryTransactionRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteInventoryTransactionRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl InventoryTransactionRepoTrait for SqliteInventoryTransactionRepo {
    fn get_by_id(&self, id: i64) -> Result<Option<InventoryTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference, created_at \
          FROM inventory_transactions WHERE customer_mdoc = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(InventoryTransaction {
                id: r.get(0)?,
                upc: r.get(1)?,
                quantity_change: r.get(2)?,
                operator_mdoc: r.get(3)?,
                customer_mdoc: r.get(4)?,
                ref_order_id: r.get(5)?,
                reference: r.get(6)?,
                created_at: r.get(7)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, a: &InventoryTransaction) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO inventory_transactions \
         (upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                a.upc,
                a.quantity_change,
                a.operator_mdoc,
                a.customer_mdoc,
                a.ref_order_id,
                a.reference
            ],
        )?;
        Ok(())
    }

    fn search(
        &self,
        limit: i64,
        offset: i64,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(InventoryTransaction, String, String)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT tx.id,
                   tx.upc,
                   tx.quantity_change,
                   tx.operator_mdoc,
                   tx.customer_mdoc,
                   tx.ref_order_id,
                   tx.reference,
                   tx.created_at,
                   p.desc AS product_name,
                   o.name AS operator_name
            FROM inventory_transactions tx
            JOIN products  p ON tx.upc           = p.upc
            JOIN operators o ON tx.operator_mdoc = o.mdoc
            WHERE tx.customer_mdoc IS NULL
              AND tx.quantity_change <= 0
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        // date filter
        if let Some(ref d) = date {
            sql.push_str(" AND date(tx.created_at)=date(?)");
            params.push(d);
        }
        // multi‑field text search
        if let Some(ref s) = search {
            sql.push_str(
                " AND (\
                tx.upc           LIKE ? OR \
                CAST(tx.operator_mdoc AS TEXT) LIKE ? OR \
                CAST(tx.ref_order_id    AS TEXT) LIKE ? OR \
                tx.reference     LIKE ? OR \
                p.desc           LIKE ? OR \
                o.name           LIKE ?\
            )",
            );
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let pat_ref = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("search pattern missing".to_string()))?;
            for _ in 0..6 {
                params.push(pat_ref);
            }
        }

        sql.push_str(" ORDER BY tx.created_at DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            let tx = InventoryTransaction {
                id: r.get(0)?,
                upc: r.get(1)?,
                quantity_change: r.get(2)?,
                operator_mdoc: r.get(3)?,
                customer_mdoc: r.get(4)?,
                ref_order_id: r.get(5)?,
                reference: r.get(6)?,
                created_at: r.get(7)?,
            };
            let product_name: String = r.get(8)?;
            let operator_name: String = r.get(9)?;
            Ok((tx, product_name, operator_name))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT COUNT(*) \
            FROM inventory_transactions tx \
            WHERE tx.customer_mdoc IS NULL \
              AND tx.quantity_change <= 0"
            .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();
        if let Some(ref d) = date {
            sql.push_str(" AND date(tx.created_at)=date(?)");
            params.push(d);
        }
        if let Some(ref s) = search {
            sql.push_str(" AND (upc LIKE ? OR operator_mdoc LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let last = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("search pattern missing".to_string()))?;
            params.push(last);
            params.push(last);
        }
        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| r.get(0))
            .map_err(Into::into)
    }

    fn list_for_customer(&self, customer_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn
            .prepare(         "SELECT id, upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference, created_at \
          FROM inventory_transactions WHERE customer_mdoc = ?1")?;
        let adjustments = stmt
            .query_map(params![customer_mdoc], |r| {
                Ok(InventoryTransaction {
                    id: r.get(0)?,
                    upc: r.get(1)?,
                    quantity_change: r.get(2)?,
                    operator_mdoc: r.get(3)?,
                    customer_mdoc: r.get(4)?,
                    ref_order_id: r.get(5)?,
                    reference: r.get(6)?,
                    created_at: r.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(adjustments)
    }

    fn list(&self) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference, created_at \
             FROM inventory_transactions"
        )?;
        let items = stmt
            .query_map([], |r| {
                Ok(InventoryTransaction {
                    id: r.get(0)?,
                    upc: r.get(1)?,
                    quantity_change: r.get(2)?,
                    operator_mdoc: r.get(3)?,
                    customer_mdoc: r.get(4)?,
                    ref_order_id: r.get(5)?,
                    reference: r.get(6)?,
                    created_at: r.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }

    fn list_for_product(&self, upc: String) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference, created_at \
             FROM inventory_transactions WHERE upc = ?1"
        )?;
        let mapped = stmt.query_map(params![upc], |r| {
            Ok(InventoryTransaction {
                id: r.get(0)?,
                upc: r.get(1)?,
                quantity_change: r.get(2)?,
                operator_mdoc: r.get(3)?,
                customer_mdoc: r.get(4)?,
                ref_order_id: r.get(5)?,
                reference: r.get(6)?,
                created_at: r.get(7)?,
            })
        })?;
        let collected = mapped.collect::<Result<Vec<_>, _>>()?;
        Ok(collected)
    }

    fn list_for_operator(&self, operator_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
         "SELECT id, upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference, created_at \
          FROM inventory_transactions WHERE operator_mdoc = ?1"
        )?;
        let mapped = stmt.query_map(params![operator_mdoc], |r| {
            Ok(InventoryTransaction {
                id: r.get(0)?,
                upc: r.get(1)?,
                quantity_change: r.get(2)?,
                operator_mdoc: r.get(3)?,
                customer_mdoc: r.get(4)?,
                ref_order_id: r.get(5)?,
                reference: r.get(6)?,
                created_at: r.get(7)?,
            })
        })?;
        let collected = mapped.collect::<Result<Vec<_>, _>>()?;
        Ok(collected)
    }

    fn list_for_today(&self) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, upc, quantity_change, operator_mdoc, customer_mdoc, ref_order_id, reference, created_at \
             FROM inventory_transactions WHERE date(created_at) = date('now')"
        )?;
        let mapped = stmt.query_map([], |r| {
            Ok(InventoryTransaction {
                id: r.get(0)?,
                upc: r.get(1)?,
                quantity_change: r.get(2)?,
                operator_mdoc: r.get(3)?,
                customer_mdoc: r.get(4)?,
                ref_order_id: r.get(5)?,
                reference: r.get(6)?,
                created_at: r.get(7)?,
            })
        })?;
        let collected = mapped.collect::<Result<Vec<_>, _>>()?;
        Ok(collected)
    }
}
