use crate::common::error::AppError;
use crate::common::mutex_ext::MutexExt;
use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteOperatorRepo {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteOperatorRepo {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl OperatorRepoTrait for SqliteOperatorRepo {
    fn get_by_mdoc(&self, mdoc: i32) -> Result<Option<Operator>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt =
            conn.prepare("SELECT mdoc, name, start, stop FROM operators WHERE mdoc = ?1")?;
        let mut rows = stmt.query(params![mdoc])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Operator {
                mdoc: r.get(0)?,
                name: r.get(1)?,
                start: r.get(2)?,
                stop: r.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, o: &Operator) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "INSERT INTO operators (mdoc, name, start, stop) VALUES (?1, ?2, ?3, ?4)",
            params![o.mdoc, o.name, o.start, o.stop],
        )?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Operator>, AppError> {
        let conn = self.conn.safe_lock()?;
        let mut stmt = conn.prepare("SELECT mdoc, name, start, stop FROM operators")?;
        let ops = stmt
            .query_map([], |r| {
                Ok(Operator {
                    mdoc: r.get(0)?,
                    name: r.get(1)?,
                    start: r.get(2)?,
                    stop: r.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ops)
    }

    fn update_by_mdoc(&self, o: &Operator) -> Result<(), AppError> {
        let conn = self.conn.safe_lock()?;
        conn.execute(
            "UPDATE operators SET name = ?1, start = ?2, stop = ?3 WHERE mdoc = ?4",
            params![o.name, o.start, o.stop, o.mdoc],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod repo_tests {
    use super::*;
    use std::sync::Arc;

    /// Helper to spin up an in‐memory repo with migrations applied
    fn make_live_repo() -> SqliteOperatorRepo {
        let mutex = crate::infrastructure::db::create_connection(":memory:").unwrap();
        let conn = mutex.into_inner().unwrap();
        SqliteOperatorRepo::new(Arc::new(Mutex::new(conn)))
    }

    #[test]
    fn get_nonexistent_and_list_on_clean_repo() {
        let repo = make_live_repo();
        // Nothing inserted yet
        assert!(repo.get_by_mdoc(42).unwrap().is_none());
        assert!(repo.list().unwrap().is_empty());
    }
}
