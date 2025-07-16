use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::ClubImport;
use crate::domain::repos::ClubImportRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteClubImportRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteClubImportRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl ClubImportRepoTrait for SqliteClubImportRepo {
    fn list(&self) -> Result<Vec<ClubImport>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, date, activity_from, activity_to, source_file
             FROM club_imports ORDER BY date DESC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(ClubImport {
                id: r.get(0)?,
                date: r.get(1)?,
                activity_from: r.get(2)?,
                activity_to: r.get(3)?,
                source_file: r.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn get_by_id(&self, id: i32) -> Result<Option<ClubImport>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare(
            "SELECT id, date, activity_from, activity_to, source_file
             FROM club_imports WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(ClubImport {
                id: r.get(0)?,
                date: r.get(1)?,
                activity_from: r.get(2)?,
                activity_to: r.get(3)?,
                source_file: r.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, import: &ClubImport) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO club_imports (date, activity_from, activity_to, source_file) VALUES (?1, ?2, ?3, ?4)",
            params![
                import.date,
                import.activity_from,
                import.activity_to,
                import.source_file
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
    ) -> Result<Vec<ClubImport>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        // build base SQL
        let mut sql = "\
            SELECT id, date, activity_from, activity_to, source_file \
            FROM club_import WHERE 1=1\
        "
        .to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        // date filter
        if let Some(ref d) = date {
            sql.push_str(" AND date(date)=date(?)");
            params.push(d);
        }
        // search on id or source_file
        if let Some(ref s) = search {
            sql.push_str(" AND (id LIKE ? OR source_file LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let p = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("club_import pattern missing".into()))?;
            params.push(p);
            params.push(p);
        }

        // ordering + pagination
        sql.push_str(" ORDER BY date DESC LIMIT ? OFFSET ?");
        params.push(&limit);
        params.push(&offset);

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params.as_slice(), |r| {
            Ok(ClubImport {
                id: r.get(0)?,
                date: r.get(1)?,
                activity_from: r.get(2)?,
                activity_to: r.get(3)?,
                source_file: r.get(4)?,
            })
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn count(&self, date: Option<String>, search: Option<String>) -> Result<i64, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut sql = "SELECT COUNT(*) FROM club_import WHERE 1=1".to_string();
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let mut string_params: Vec<String> = Vec::new();

        if let Some(ref d) = date {
            sql.push_str(" AND date(date)=date(?)");
            params.push(d);
        }
        if let Some(ref s) = search {
            sql.push_str(" AND (id LIKE ? OR source_file LIKE ?)");
            let pat = format!("%{}%", s);
            string_params.push(pat);
            let p = string_params
                .last()
                .ok_or_else(|| AppError::Unexpected("club_import count pattern missing".into()))?;
            params.push(p);
            params.push(p);
        }

        let mut stmt = conn.prepare(&sql)?;
        stmt.query_row(params.as_slice(), |r| r.get(0))
            .map_err(Into::into)
    }
}
