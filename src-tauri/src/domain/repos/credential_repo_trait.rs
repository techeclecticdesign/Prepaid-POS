use crate::common::error::AppError;

pub trait CredentialRepoTrait: Send + Sync {
    fn set_password(&self, hash: &str) -> Result<(), AppError>;
    fn get_password_hash(&self) -> Result<Option<String>, AppError>;
    fn delete_password(&self) -> Result<(), AppError>;
}
