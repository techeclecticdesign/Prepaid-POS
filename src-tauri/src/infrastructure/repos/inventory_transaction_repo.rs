use crate::common::error::AppError;
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
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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

    fn list_for_customer(&self, customer_mdoc: i32) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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

    fn list_for_product(&self, upc: i64) -> Result<Vec<InventoryTransaction>, AppError> {
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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
        let conn = self.conn.lock().unwrap();
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
