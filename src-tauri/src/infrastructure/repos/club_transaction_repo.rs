use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::ClubTransaction;
use crate::domain::repos::ClubTransactionRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteClubTransactionRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteClubTransactionRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl ClubTransactionRepoTrait for SqliteClubTransactionRepo {
    fn list(&self) -> Result<Vec<ClubTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, import_id, entity_name, mdoc, tx_type, amount, date FROM club_transactions ORDER BY date DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(ClubTransaction {
                id: r.get(0)?,
                import_id: r.get(1)?,
                entity_name: r.get(2)?,
                mdoc: r.get(3)?,
                tx_type: r.get(4)?,
                amount: r.get(5)?,
                date: r.get(6)?,
            })
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_by_id(&self, id: i32) -> Result<Option<ClubTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, import_id, entity_name, mdoc, tx_type, amount, date FROM club_transactions WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(ClubTransaction {
                id: r.get(0)?,
                import_id: r.get(1)?,
                entity_name: r.get(2)?,
                mdoc: r.get(3)?,
                tx_type: r.get(4)?,
                amount: r.get(5)?,
                date: r.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, tx: &ClubTransaction) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO club_transactions (import_id, entity_name, mdoc, tx_type, amount, date) VALUES (?1, ?2, ?3, ?4)",
            params![tx.import_id, tx.entity_name, tx.mdoc, format!("{:?}", tx.tx_type), tx.amount, tx.date],
        )?;
        Ok(())
    }
}
