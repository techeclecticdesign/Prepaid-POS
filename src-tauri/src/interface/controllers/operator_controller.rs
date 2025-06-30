use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use crate::error::AppError;
use crate::services::operator_service::OperatorService;
use std::sync::Arc;

/// Controller knows how to orchestrate a use‑case.
pub struct OperatorController {
    svc: OperatorService,
}

impl OperatorController {
    pub fn new(repo: Arc<dyn OperatorRepoTrait>) -> Self {
        Self {
            svc: OperatorService::new(repo),
        }
    }

    pub fn list(&self) -> Result<Vec<Operator>, AppError> {
        self.svc.list_operators()
    }

    pub fn get(&self, id: i32) -> Result<Option<Operator>, AppError> {
        self.svc.get_operator(id)
    }

    pub fn create(&self, op: Operator) -> Result<(), AppError> {
        self.svc.create_operator(&op)
    }

    pub fn update(&self, op: Operator) -> Result<(), AppError> {
        self.svc.update_operator(&op)
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_operator_repo::MockOperatorRepo;
    use std::sync::Arc;

    #[test]
    fn controller_smoke_list() {
        let repo = Arc::new(MockOperatorRepo::new());
        let ctrl = OperatorController::new(repo.clone());
        // empty list comes back okay
        let out = ctrl.list().expect("list should succeed");
        assert!(out.is_empty());
    }
}
