use crate::common::error::AppError;
use crate::try_log;
use std::sync::Arc;

pub struct StatsUseCases {
    club_tx_repo: Arc<dyn crate::domain::repos::ClubTransactionRepoTrait>,
    customer_repo: Arc<dyn crate::domain::repos::CustomerRepoTrait>,
}

impl StatsUseCases {
    pub fn new(
        club_tx_repo: Arc<dyn crate::domain::repos::ClubTransactionRepoTrait>,
        customer_repo: Arc<dyn crate::domain::repos::CustomerRepoTrait>,
    ) -> Self {
        Self {
            club_tx_repo,
            customer_repo,
        }
    }

    pub fn get_stats(&self) -> Result<(i64, i64), AppError> {
        let account_total = try_log!(
            self.club_tx_repo.get_account_total(),
            "StatsUseCases::get_stats::get_account_total"
        );

        let total_customer_balances = try_log!(
            self.customer_repo.sum_all_balances(),
            "StatsUseCases::get_stats::sum_all_balances"
        );

        Ok((account_total as i64, total_customer_balances as i64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::mock_club_tx_repo::MockClubTransactionRepo;
    use crate::test_support::mock_customer_repo::MockCustomerRepo;
    use std::sync::Arc;

    fn make_usecase() -> StatsUseCases {
        let club_repo = Arc::new(MockClubTransactionRepo::new());
        let cust_repo = Arc::new(MockCustomerRepo::new());
        StatsUseCases::new(club_repo, cust_repo)
    }

    #[test]
    fn smoke_get_stats() {
        let uc = make_usecase();
        let (acct, balances) = uc.get_stats().expect("get_stats should succeed");
        assert_eq!(acct, 0);
        assert_eq!(balances, 0);
    }
}
