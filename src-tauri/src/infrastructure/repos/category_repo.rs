use crate::common::error::AppError;
use crate::domain::models::Category;
use crate::domain::repos::CategoryRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteCategoryRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCategoryRepo {
    pub const fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl CategoryRepoTrait for SqliteCategoryRepo {
    fn list(&self) -> Result<Vec<Category>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT id, name, deleted FROM categories")?;
        let rows = stmt.query_map([], |r| {
            Ok(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                deleted: r.get(2)?,
            })
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn list_active(&self) -> Result<Vec<Category>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut stmt =
            conn.prepare("SELECT id, name, deleted FROM categories WHERE deleted IS NULL")?;
        let rows = stmt.query_map([], |r| {
            Ok(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                deleted: r.get(2)?,
            })
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    fn get_by_id(&self, id: i32) -> Result<Option<Category>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT id, name, deleted FROM categories WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                deleted: r.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, c: String) -> Result<(), AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        conn.execute("INSERT INTO categories (name) VALUES (?1)", params![c])?;
        Ok(())
    }

    fn soft_delete(&self, id: i32) -> Result<(), AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        conn.execute(
            "UPDATE categories SET deleted = datetime('now') WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    fn get_by_name(&self, name: &str) -> Result<Option<Category>, AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT id, name, deleted FROM categories WHERE name = ?1")?;
        let mut rows = stmt.query(params![name])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                deleted: r.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn undelete(&self, id: i32) -> Result<(), AppError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AppError::LockPoisoned(e.to_string()))?;
        conn.execute(
            "UPDATE categories SET deleted = NULL WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}
