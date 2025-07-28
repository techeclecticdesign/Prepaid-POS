use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::repos::WeeklyLimitRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteWeeklyLimitRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteWeeklyLimitRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl WeeklyLimitRepoTrait for SqliteWeeklyLimitRepo {
    fn set_limit(&self, amount: i32) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute("DELETE FROM weekly_limit", [])?;
        conn.execute(
            "INSERT INTO weekly_limit(amount) VALUES(?1)",
            params![amount],
        )?;
        Ok(())
    }

    fn get_limit(&self) -> Result<i32, AppError> {
        let conn = self.conn.safe_lock()?;
        conn.query_row("SELECT amount FROM weekly_limit", [], |row| row.get(0))
            .map_err(Into::into)
    }
}
