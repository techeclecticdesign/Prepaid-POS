use crate::application::use_cases::auth_usecases::AuthUseCase;
use crate::common::auth::AuthState;
use crate::common::error::AppError;
use crate::common::rwlock_ext::RwLockExt;
use crate::domain::repos::CredentialRepoTrait;
use std::sync::{Arc, RwLock};

pub struct AuthController {
    state: Arc<RwLock<AuthState>>,
    uc: AuthUseCase,
}

impl AuthController {
    pub fn new(state: Arc<RwLock<AuthState>>, cred_repo: Arc<dyn CredentialRepoTrait>) -> Self {
        Self {
            state,
            uc: AuthUseCase::new(cred_repo),
        }
    }

    // checks the password and sets auth state if valid
    pub fn login(&self, password: String) -> Result<(), AppError> {
        // If no password is set, skip auth
        if !self.uc.password_required()? || self.uc.validate_password(&password)? {
            let mut st = self.state.safe_write().map_err(AppError::LockPoisoned)?;
            st.logged_in = true;
            st.last_activity = Some(std::time::Instant::now());
            Ok(())
        } else {
            Err(AppError::Unauthorized)
        }
    }

    // clears auth state
    pub fn logout(&self) -> Result<(), AppError> {
        let mut st = self.state.safe_write().map_err(AppError::LockPoisoned)?;
        st.logged_in = false;
        st.last_activity = None;
        Ok(())
    }

    // checks timeout & returns current status
    pub fn check_status(&self) -> Result<bool, AppError> {
        let mut st = self.state.safe_write().map_err(AppError::LockPoisoned)?;
        if st.logged_in {
            if let Some(last) = st.last_activity {
                if last.elapsed().as_secs() > 60 * 15 {
                    st.logged_in = false;
                    st.last_activity = None;
                    return Ok(false);
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    // bump the last_activity timestamp
    pub fn update_activity(&self) -> Result<(), AppError> {
        let mut st = self.state.safe_write().map_err(AppError::LockPoisoned)?;
        if st.logged_in {
            st.last_activity = Some(std::time::Instant::now());
        }
        Ok(())
    }

    pub fn change_password(&self, old: String, new: String) -> Result<(), AppError> {
        // If a password exists, enforce old‑password check; otherwise skip
        if self.uc.password_required()? && !self.uc.validate_password(&old)? {
            return Err(AppError::Unauthorized);
        }
        self.uc.set_password(&new)
    }

    pub fn password_required(&self) -> Result<bool, AppError> {
        self.uc.password_required()
    }

    pub fn delete_password(&self) -> Result<(), AppError> {
        // clear any logged-in state
        let mut st = self.state.safe_write().map_err(AppError::LockPoisoned)?;
        st.logged_in = true;
        st.last_activity = None;
        self.uc.delete_password()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::auth::AuthState;
    use crate::common::error::AppError;
    use crate::test_support::mock_cred_repo::MockCredRepo;
    use std::sync::{Arc, RwLock};

    fn setup() -> AuthController {
        let repo = Arc::new(MockCredRepo::new());
        // seed with a known password
        let uc = AuthUseCase::new(repo.clone());
        uc.set_password("init").unwrap();

        let state = Arc::new(RwLock::new(AuthState::default()));
        AuthController::new(state, repo)
    }

    #[test]
    fn login_logout_flow() {
        let ctrl = setup();

        // wrong password should error
        assert!(matches!(
            ctrl.login("bad".into()),
            Err(AppError::Unauthorized)
        ));

        // correct password logs in
        ctrl.login("init".into()).unwrap();
        assert!(ctrl.check_status().unwrap());

        // logout clears
        ctrl.logout().unwrap();
        assert!(!ctrl.check_status().unwrap());
    }

    #[test]
    fn change_password_requires_old() {
        let ctrl = setup();

        // try changing with bad old pw
        let err = ctrl
            .change_password("wrong".into(), "new".into())
            .unwrap_err();
        assert!(matches!(err, AppError::Unauthorized));

        // now change with correct old pw
        ctrl.change_password("init".into(), "new".into()).unwrap();

        // old no longer works, new does
        assert!(!ctrl
            .login("init".into())
            .unwrap_err()
            .to_string()
            .contains("Invalid"));
        ctrl.logout().unwrap();
        ctrl.login("new".into()).unwrap();
        assert!(ctrl.check_status().unwrap());
    }

    #[test]
    fn login_when_no_password() {
        // Create a repo with no password ever set
        let repo = Arc::new(MockCredRepo::new());
        let state = Arc::new(RwLock::new(AuthState::default()));
        let ctrl = AuthController::new(state, repo);

        // Should succeed even with empty input
        assert!(ctrl.login("anything".into()).is_ok());
        assert!(ctrl.check_status().unwrap());
    }
}
