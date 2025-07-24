use crate::common::auth::{hash_password, verify_password};
use crate::common::error::AppError;
use crate::domain::repos::CredentialRepoTrait;
use std::sync::Arc;

pub struct AuthUseCase {
    repo: Arc<dyn CredentialRepoTrait>,
}

impl AuthUseCase {
    pub fn new(repo: Arc<dyn CredentialRepoTrait>) -> Self {
        Self { repo }
    }

    pub fn set_password(&self, raw: &str) -> Result<(), AppError> {
        let hash = hash_password(raw)?;
        self.repo.set_password(&hash)
    }

    pub fn validate_password(&self, raw: &str) -> Result<bool, AppError> {
        if let Some(hash) = self.repo.get_password_hash()? {
            Ok(verify_password(&hash, raw))
        } else {
            Ok(false)
        }
    }

    pub fn password_required(&self) -> Result<bool, AppError> {
        Ok(self.repo.get_password_hash()?.is_some())
    }

    pub fn delete_password(&self) -> Result<(), AppError> {
        self.repo.delete_password()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::mock_cred_repo::MockCredRepo;
    use std::sync::Arc;

    #[test]
    fn test_set_and_validate_password() {
        let repo = Arc::new(MockCredRepo::new());
        let uc = AuthUseCase::new(repo.clone());

        // initially no pw set
        assert_eq!(uc.validate_password("foo").unwrap(), false);

        // set a password
        uc.set_password("secret").unwrap();

        // correct raw should validate
        assert!(uc.validate_password("secret").unwrap());

        // wrong raw should not
        assert!(!uc.validate_password("wrong").unwrap());
    }

    #[test]
    fn test_hash_is_unique_per_call() {
        let repo = Arc::new(MockCredRepo::new());
        let uc = AuthUseCase::new(repo.clone());

        uc.set_password("dup").unwrap();
        let first = repo.get_password_hash().unwrap().unwrap();
        uc.set_password("dup").unwrap();
        let second = repo.get_password_hash().unwrap().unwrap();

        // Argon2 salts uniquely, so two hashes should differ
        assert_ne!(first, second);
    }

    #[test]
    fn test_password_required_logic() {
        let repo = Arc::new(MockCredRepo::new());
        let uc = AuthUseCase::new(repo.clone());

        // No password set → not required
        assert_eq!(uc.password_required().unwrap(), false);

        // Once we set it, it should be required
        uc.set_password("x").unwrap();
        assert_eq!(uc.password_required().unwrap(), true);
    }
}
