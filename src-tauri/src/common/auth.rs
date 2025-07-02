use std::time::Instant;

#[derive(Default)]
pub struct AuthState {
    pub logged_in: bool,
    pub last_activity: Option<Instant>,
}
