use crate::application::use_cases::stats_usecases::StatsUseCases;
use crate::common::error::AppError;
use crate::interface::dto::stats_dto::StatsDto;
use std::sync::Arc;

pub struct StatsController {
    uc: StatsUseCases,
}

impl StatsController {
    pub fn new(
        club_tx_repo: Arc<dyn crate::domain::repos::ClubTransactionRepoTrait>,
        customer_repo: Arc<dyn crate::domain::repos::CustomerRepoTrait>,
    ) -> Self {
        Self {
            uc: StatsUseCases::new(club_tx_repo, customer_repo),
        }
    }

    pub fn get_stats(&self) -> Result<StatsDto, AppError> {
        let (account_total, total_customer_balances) = self.uc.get_stats()?;
        Ok(StatsDto {
            account_total,
            total_customer_balances,
        })
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::test_support::mock_club_tx_repo::MockClubTransactionRepo;
    use crate::test_support::mock_customer_repo::MockCustomerRepo;

    fn make_controller() -> StatsController {
        let club_repo = Arc::new(MockClubTransactionRepo::new());
        let cust_repo = Arc::new(MockCustomerRepo::new());
        StatsController::new(club_repo, cust_repo)
    }

    #[test]
    fn smoke_get_stats_returns_zero_for_empty_repos() {
        let ctrl = make_controller();
        let out = ctrl.get_stats().expect("get_stats should succeed");
        assert_eq!(out.account_total, 0);
        assert_eq!(out.total_customer_balances, 0);
    }
}
