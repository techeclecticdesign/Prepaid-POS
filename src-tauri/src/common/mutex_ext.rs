use crate::common::error::AppError;
use std::sync::{Mutex, MutexGuard};

pub trait MutexExt<T> {
    fn safe_lock(&self) -> Result<MutexGuard<'_, T>, AppError>;
}

impl<T> MutexExt<T> for Mutex<T> {
    fn safe_lock(&self) -> Result<MutexGuard<'_, T>, AppError> {
        match self.lock() {
            Ok(guard) => Ok(guard),
            Err(poisoned) => {
                log::error!("Mutex poisoned: {poisoned}");
                Err(AppError::LockPoisoned("Mutex was poisoned".into()))
            }
        }
    }
}

#[test]
fn poisoned_mutex_does_not_panic() {
    use crate::common::mutex_ext::MutexExt;
    use std::sync::{Arc, Mutex};
    use std::thread;

    let m = Arc::new(Mutex::new(42));

    // poison it
    {
        let m2 = Arc::clone(&m);
        let _ = thread::spawn(move || {
            let _guard = m2.lock().unwrap();
            panic!("simulate panic while holding lock");
        })
        .join();
    }

    // Now the mutex is poisoned; our safe_lock should not panic but return Err
    let result = m.safe_lock();
    assert!(result.is_err(), "Expected error due to poisoned mutex");
}
