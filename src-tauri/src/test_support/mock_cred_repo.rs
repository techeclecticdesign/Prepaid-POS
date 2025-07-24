use crate::common::error::AppError;
use crate::domain::repos::CredentialRepoTrait;
use std::sync::Mutex;

pub struct MockCredRepo {
    inner: Mutex<Option<String>>,
}

impl Default for MockCredRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl MockCredRepo {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    pub fn get_inner(&self) -> Option<String> {
        self.inner.lock().unwrap().clone()
    }
}

impl CredentialRepoTrait for MockCredRepo {
    fn set_password(&self, hash: &str) -> Result<(), AppError> {
        *self.inner.lock().unwrap() = Some(hash.to_owned());
        Ok(())
    }

    fn get_password_hash(&self) -> Result<Option<String>, AppError> {
        Ok(self.inner.lock().unwrap().clone())
    }

    fn delete_password(&self) -> Result<(), AppError> {
        *self.inner.lock().unwrap() = None;
        Ok(())
    }
}
