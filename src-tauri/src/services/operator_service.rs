use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use log::{error, info, warn};
use std::sync::Arc;

pub struct OperatorService {
    repo: Arc<dyn OperatorRepoTrait>,
}

impl OperatorService {
    /// Inject the repo
    pub fn new(repo: Arc<dyn OperatorRepoTrait>) -> Self {
        Self { repo }
    }

    pub fn list_operators(&self) -> anyhow::Result<Vec<Operator>> {
        self.repo.list()
    }

    pub fn get_operator(&self, id: i32) -> anyhow::Result<Option<Operator>> {
        self.repo.get_by_id(id)
    }

    pub fn create_operator(&self, op: &Operator) -> anyhow::Result<()> {
        if op.name.trim().is_empty() {
            warn!("create failed: name empty (name={})", op.name);
            anyhow::bail!("Operator name cannot be empty");
        }
        // Check if any operator already has this id
        let existing = self.repo.list()?.into_iter().find(|o| o.id == op.id);
        if existing.is_some() {
            warn!("create failed: duplicate id {}", op.id);
            anyhow::bail!("Operator id '{}' already exists", op.id);
        }
        let res = self.repo.create(op);
        match &res {
            Ok(()) => info!("operator created: id={} name={}", op.id, op.name),
            Err(e) => error!("operator create error: id={} error={}", op.id, e),
        }
        res
    }

    /// Update an existing operator
    pub fn update_operator(&self, op: &Operator) -> anyhow::Result<()> {
        // Check if operator exists
        let existing = self.repo.get_by_id(op.id)?;
        if existing.is_none() {
            warn!("update failed: not found (id={})", op.id);
            anyhow::bail!("Cannot update: Operator with id {} not found", op.id);
        }
        let res = self.repo.update_by_id(op);
        match &res {
            Ok(()) => info!("operator updated: id={} name={}", op.id, op.name),
            Err(e) => error!("operator update error: id={} error={}", op.id, e),
        }
        res
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
        let svc = OperatorService::new(mock.clone());

        // initially empty
        assert!(svc.list_operators()?.is_empty());

        // create operator
        let op = Operator {
            id: 1,
            name: "Alice".into(),
            start: chrono::Utc::now().naive_utc(),
            stop: None,
        };
        svc.create_operator(&op)?;
        assert_eq!(svc.list_operators()?, vec![op.clone()]);

        // update operator
        let mut updated = op.clone();
        updated.name = "Alice Updated".into();
        svc.update_operator(&updated)?;
        assert_eq!(svc.get_operator(1)?.unwrap().name, "Alice Updated");

        Ok(())
    }

    #[test]
    fn create_duplicate_id_error() {
        let mock = Arc::new(MockOperatorRepo::new());
        let svc = OperatorService::new(mock.clone());

        let op1 = Operator {
            id: 1,
            name: "Sibley".into(),
            start: chrono::Utc::now().naive_utc(),
            stop: None,
        };
        svc.create_operator(&op1).unwrap();

        let op_dup = Operator {
            id: 1,
            name: "Bubar".into(),
            start: chrono::Utc::now().naive_utc(),
            stop: None,
        };
        let err = svc.create_operator(&op_dup).unwrap_err();
        assert!(err.to_string().contains("Operator id '1' already exists"));
    }

    #[test]
    fn update_nonexistent_operator_error() {
        let mock = Arc::new(MockOperatorRepo::new());
        let svc = OperatorService::new(mock.clone());

        let op = Operator {
            id: 99,
            name: "Ghost".into(),
            start: chrono::Utc::now().naive_utc(),
            stop: None,
        };
        let err = svc.update_operator(&op).unwrap_err();
        assert!(err.to_string().contains("Operator with id 99 not found"));
    }

    #[test]
    fn create_empty_name_error() {
        let mock = Arc::new(MockOperatorRepo::new());
        let svc = OperatorService::new(mock.clone());

        let op = Operator {
            id: 1,
            name: "".into(),
            start: chrono::Utc::now().naive_utc(),
            stop: None,
        };
        let err = svc.create_operator(&op).unwrap_err();
        assert!(err.to_string().contains("Operator name cannot be empty"));
    }
}
