use crate::common::error::AppError;
use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use crate::try_log;
use log::{info, warn};
use std::sync::Arc;

pub struct OperatorUseCases {
    repo: Arc<dyn OperatorRepoTrait>,
}

impl OperatorUseCases {
    pub fn new(repo: Arc<dyn OperatorRepoTrait>) -> Self {
        Self { repo }
    }

    pub fn list_operators(&self) -> Result<Vec<Operator>, AppError> {
        let res = try_log!(self.repo.list(), "OperatorUseCases::list_operators");
        Ok(res)
    }

    pub fn create_operator(&self, op: &Operator) -> Result<(), AppError> {
        if op.name.trim().is_empty() {
            warn!("create failed: name empty (name={})", op.name);
            return Err(AppError::Unexpected("Operator name cannot be empty".into()));
        }
        // Check if any operator already has this mdoc
        let existing = try_log!(self.repo.list(), "OperatorUseCases::create_operator")
            .into_iter()
            .find(|o| o.mdoc == op.mdoc);
        if existing.is_some() {
            warn!("create failed: duplicate mdoc {}", op.mdoc);
            return Err(AppError::Unexpected(format!(
                "Operator mdoc '{}' already exists",
                op.mdoc
            )));
        }
        try_log!(self.repo.create(op), "OperatorUseCases::create_operator");
        info!("operator created: mdoc={} name={}", op.mdoc, op.name);
        Ok(())
    }

    pub fn update_operator(&self, op: &Operator) -> Result<(), AppError> {
        // Check if operator exists
        let existing = try_log!(
            self.repo.get_by_mdoc(op.mdoc),
            "OperatorUseCases::update_operator"
        );
        if existing.is_none() {
            warn!("update failed: not found (mdoc={})", op.mdoc);
            return Err(AppError::NotFound(format!(
                "Cannot update: Operator with mdoc {} not found",
                op.mdoc
            )));
        }
        try_log!(
            self.repo.update_by_mdoc(op),
            "OperatorUseCases::update_operator"
        );
        info!("operator updated: mdoc={} name={}", op.mdoc, op.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::mock_operator_repo::MockOperatorRepo;
    use std::sync::Arc;

    #[test]
    fn service_crud_flow() -> anyhow::Result<()> {
        let mock = Arc::new(MockOperatorRepo::new());
        let uc = OperatorUseCases::new(mock.clone());

        // initially empty
        assert!(uc.list_operators()?.is_empty());

        // create operator
        let op = Operator {
            mdoc: 1,
            name: "Alice".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        };
        uc.create_operator(&op)?;
        assert_eq!(uc.list_operators()?, vec![op.clone()]);

        Ok(())
    }

    #[test]
    fn create_duplicate_id_error() {
        let mock = Arc::new(MockOperatorRepo::new());
        let uc = OperatorUseCases::new(mock.clone());

        let op1 = Operator {
            mdoc: 1,
            name: "Sibley".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        };
        uc.create_operator(&op1).unwrap();

        let op_dup = Operator {
            mdoc: 1,
            name: "Bubar".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        };
        let err = uc.create_operator(&op_dup).unwrap_err();
        assert!(err.to_string().contains("Operator mdoc '1' already exists"));
    }

    #[test]
    fn update_nonexistent_operator_error() {
        let mock = Arc::new(MockOperatorRepo::new());
        let uc = OperatorUseCases::new(mock.clone());

        let op = Operator {
            mdoc: 99,
            name: "Ghost".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        };
        let err = uc.update_operator(&op).unwrap_err();
        assert!(err.to_string().contains("Operator with mdoc 99 not found"));
    }

    #[test]
    fn create_empty_name_error() {
        let mock = Arc::new(MockOperatorRepo::new());
        let uc = OperatorUseCases::new(mock.clone());

        let op = Operator {
            mdoc: 1,
            name: "".into(),
            start: Some(chrono::Utc::now().naive_utc()),
            stop: None,
        };
        let err = uc.create_operator(&op).unwrap_err();
        assert!(err.to_string().contains("Operator name cannot be empty"));
    }
}
