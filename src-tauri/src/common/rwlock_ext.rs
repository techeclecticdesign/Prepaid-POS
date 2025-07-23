use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// trait for RwLock to prevent panics and log poisoned state
pub trait RwLockExt<T> {
    #[allow(dead_code)]
    fn safe_read(&self) -> Result<RwLockReadGuard<'_, T>, String>;
    fn safe_write(&self) -> Result<RwLockWriteGuard<'_, T>, String>;
}

impl<T> RwLockExt<T> for RwLock<T> {
    fn safe_read(&self) -> Result<RwLockReadGuard<'_, T>, String> {
        match self.read() {
            Ok(g) => Ok(g),
            Err(e) => {
                log::error!("RwLock read poisoned: {e}");
                Err("RwLock read lock poisoned".to_string())
            }
        }
    }

    fn safe_write(&self) -> Result<RwLockWriteGuard<'_, T>, String> {
        match self.write() {
            Ok(g) => Ok(g),
            Err(e) => {
                log::error!("RwLock write poisoned: {e}");
                Err("RwLock write lock poisoned".to_string())
            }
        }
    }
}
