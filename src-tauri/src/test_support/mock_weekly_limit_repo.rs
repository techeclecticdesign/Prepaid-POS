use crate::common::error::AppError;
use crate::domain::repos::WeeklyLimitRepoTrait;
use std::sync::Mutex;

pub struct MockWeeklyLimitRepo {
    inner: Mutex<i32>,
}

impl MockWeeklyLimitRepo {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(0),
        }
    }
}

impl Default for MockWeeklyLimitRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl WeeklyLimitRepoTrait for MockWeeklyLimitRepo {
    fn set_limit(&self, amount: i32) -> Result<(), AppError> {
        *self.inner.lock().unwrap() = amount;
        Ok(())
    }

    fn get_limit(&self) -> Result<i32, AppError> {
        Ok(*self.inner.lock().unwrap())
    }
}
