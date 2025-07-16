use crate::application::use_cases::operator_usecases::OperatorUseCases;
use crate::common::error::AppError;
use crate::domain::models::Operator;
use crate::domain::repos::OperatorRepoTrait;
use crate::interface::common::date_utils::parse_rfc3339;
use crate::interface::dto::operator_dto::OperatorDto;
use std::sync::Arc;
use validator::Validate;

pub struct OperatorController {
    uc: OperatorUseCases,
}

impl OperatorController {
    pub fn new(repo: Arc<dyn OperatorRepoTrait>) -> Self {
        Self {
            uc: OperatorUseCases::new(repo),
        }
    }

    pub fn list(&self) -> Result<Vec<OperatorDto>, AppError> {
        let domains = self.uc.list_operators()?;
        Ok(
            crate::interface::presenters::operator_presenter::OperatorPresenter::to_dto_list(
                domains,
            ),
        )
    }

    pub fn get(&self, id: i32) -> Result<Option<OperatorDto>, AppError> {
        let opt = self.uc.get_operator(id)?;
        Ok(opt.map(crate::interface::presenters::operator_presenter::OperatorPresenter::to_dto))
    }

    pub fn create(&self, dto: OperatorDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        let start = match &dto.start {
            Some(s) => Some(parse_rfc3339(s)?),
            None => None,
        };
        let stop = match &dto.stop {
            Some(s) => Some(parse_rfc3339(s)?),
            None => None,
        };
        let op = Operator {
            mdoc: dto.mdoc,
            name: dto.name,
            start,
            stop,
        };
        self.uc.create_operator(&op)
    }

    pub fn update(&self, dto: OperatorDto) -> Result<(), AppError> {
        dto.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;
        let start = match &dto.start {
            Some(s) => Some(parse_rfc3339(s)?),
            None => None,
        };
        let stop = match &dto.stop {
            Some(s) => Some(parse_rfc3339(s)?),
            None => None,
        };
        let op = Operator {
            mdoc: dto.mdoc,
            name: dto.name,
            start,
            stop,
        };
        self.uc.update_operator(&op)
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
