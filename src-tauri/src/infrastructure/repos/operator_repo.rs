use crate::common::error::AppError;
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
    fn get_by_id(&self, id: i32) -> Result<Option<Operator>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, start, stop FROM operators WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            Ok(Some(Operator {
                id: r.get(0)?,
                name: r.get(1)?,
                start: r.get(2)?,
                stop: r.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    fn create(&self, o: &Operator) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO operators (id, name, start, stop) VALUES (?1, ?2, ?3, ?4)",
            params![o.id, o.name, o.start, o.stop],
        )?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Operator>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, start, stop FROM operators")?;
        let ops = stmt
            .query_map([], |r| {
                Ok(Operator {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    start: r.get(2)?,
                    stop: r.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ops)
    }

    fn update_by_id(&self, o: &Operator) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE operators SET name = ?1, start = ?2, stop = ?3 WHERE id = ?4",
            params![o.name, o.start, o.stop, o.id],
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
        assert!(repo.get_by_id(42).unwrap().is_none());
        assert!(repo.list().unwrap().is_empty());
    }
}
