use crate::common::error::AppError;
use std::sync::{Arc, Mutex};

// Runs multiple repository operations atomically in a single transaction.
#[allow(dead_code)]
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
