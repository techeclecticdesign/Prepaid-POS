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
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
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
            "INSERT INTO club_transactions (import_id, entity_name, mdoc, tx_type, amount, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![tx.import_id, tx.entity_name, tx.mdoc, format!("{:?}", tx.tx_type), tx.amount, tx.date],
        )?;
        Ok(())
    }

    fn search(
        &self,
        limit: i32,
        offset: i32,
        date: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<(ClubTransaction, Option<String>)>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "\
            SELECT t.id,
                   t.import_id,
                   t.entity_name,
                   t.mdoc,
                   t.tx_type,
                   t.amount,
                   t.date,
                   c.name     AS customer_name
            FROM club_transactions t
            LEFT JOIN customer c ON t.mdoc = c.mdoc
            WHERE 1=1
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();
        if let Some(ref d) = date {
            sql.push_str(" AND date(t.date)=date(?)");
            params.push(d);
        }
        // date filter
        if let Some(ref s) = search {
            sql.push_str(" AND (c.name LIKE ? OR t.mdoc LIKE ?)");
            let pat = format!("%{s}%");
            string_params.push(pat);
            let last = string_params.last().unwrap();
            params.push(last);
            params.push(last);
        }
        sql.push_str(" ORDER BY t.date DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            let ct = ClubTransaction {
                id: r.get(0)?,
                import_id: r.get(1)?,
                entity_name: r.get(2)?,
                mdoc: r.get(3)?,
                tx_type: r.get(4)?,
                amount: r.get(5)?,
                date: r.get(6)?,
            };
            let name: Option<String> = r.get(7)?;
            Ok((ct, name))
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i32, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "SELECT COUNT(*) FROM club_transactions t LEFT JOIN customer c ON t.mdoc=c.mdoc WHERE 1=1".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();
        if let Some(ref d) = date {
            sql.push_str(" AND date(t.date)=date(?)");
            params.push(d);
        }
        if let Some(ref s) = search {
            sql.push_str(" AND (c.name LIKE ? OR t.mdoc LIKE ?)");
            let pat = format!("%{s}%");
            string_params.push(pat);
            let last = string_params.last().unwrap();
            params.push(last);
            params.push(last);
        }
        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| {
            let count: i64 = r.get(0)?;
            Ok(count as i32)
        })
        .map_err(Into::into)
    }
}
