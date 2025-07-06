use crate::common::error::AppError;
use std::sync::{Arc, Mutex};

// Runs multiple repository operations atomically in a single transaction.
pub fn atomic_tx<F, T>(conn: &Arc<Mutex<rusqlite::Connection>>, f: F) -> Result<T, AppError>
where
    F: FnOnce(&rusqlite::Transaction) -> Result<T, AppError>,
{
    let mut conn = conn.lock().unwrap();
    let tx = conn.transaction()?;
    let res = f(&tx)?;
    tx.commit()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repos::OperatorRepoTrait;
    use crate::infrastructure::db::create_connection;
    use crate::infrastructure::repos::SqliteOperatorRepo;
    use chrono::Utc;
    use rusqlite::params;
    use std::sync::Arc;

    #[test]
    fn atomic_tx_rolls_back_on_error() -> anyhow::Result<()> {
        // Set up an in‐memory DB with operators table migrated
        let conn = Arc::new(create_connection(":memory:")?);
        let repo = SqliteOperatorRepo::new(Arc::clone(&conn));

        // Sanity check: repo starts empty
        assert!(repo.list()?.is_empty());

        // Run an atomic_tx that will fail on the second insert
        let result = atomic_tx(&conn, |tx| {
            // Insert mock operator
            tx.execute(
                "INSERT INTO operators (id, name, start, stop) VALUES (?1, ?2, ?3, ?4)",
                params![
                    1,
                    "Alice",
                    Utc::now().naive_utc(),
                    Option::<chrono::NaiveDateTime>::None
                ],
            )?;
            // Attempt duplicate insert (primary‐key violation)
            tx.execute(
                "INSERT INTO operators (id, name, start, stop) VALUES (?1, ?2, ?3, ?4)",
                params![
                    1,
                    "Bob",
                    Utc::now().naive_utc(),
                    Option::<chrono::NaiveDateTime>::None
                ],
            )?;
            Ok(())
        });

        assert!(result.is_err());

        // because of rollback, no operator should be persisted
        let all = repo.list()?;
        assert!(all.is_empty(), "Expected rollback, but found {:?}", all);

        Ok(())
    }
}
