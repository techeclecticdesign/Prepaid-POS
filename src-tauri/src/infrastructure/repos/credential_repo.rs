use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::repos::CredentialRepoTrait;
use rusqlite::{params, Connection, OptionalExtension};
use std::sync::{Arc, Mutex};

pub struct SqliteCredentialRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCredentialRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl CredentialRepoTrait for SqliteCredentialRepo {
    fn set_password(&self, hash: &str) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO credentials(key,value) VALUES('system_password', ?1)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![hash],
        )?;
        Ok(())
    }

    fn get_password_hash(&self) -> Result<Option<String>, AppError> {
        let conn = self.conn.safe_lock()?;
        conn.query_row(
            "SELECT value FROM credentials WHERE key = 'system_password'",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(Into::into)
    }

    fn delete_password(&self) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute("DELETE FROM credentials WHERE key = 'system_password'", [])?;
        Ok(())
    }
}
