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
}
