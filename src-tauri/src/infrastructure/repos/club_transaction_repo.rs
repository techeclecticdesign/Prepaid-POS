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
            "SELECT id, mdoc, tx_type, amount, date FROM club_transactions ORDER BY date DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(ClubTransaction {
                id: r.get(0)?,
                mdoc: r.get(1)?,
                tx_type: r.get(2)?,
                amount: r.get(3)?,
                date: r.get(4)?,
            })
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_by_id(&self, id: i32) -> Result<Option<ClubTransaction>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, mdoc, tx_type, amount, date FROM club_transactions WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(ClubTransaction {
                id: r.get(0)?,
                mdoc: r.get(1)?,
                tx_type: r.get(2)?,
                amount: r.get(3)?,
                date: r.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }
}
