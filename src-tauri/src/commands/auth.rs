use crate::auth;
use std::env;
use std::sync::RwLock;
use tauri::State;

// Attempts to log in staff by comparing the provided password
// against the BCRYPT_HASH env var. If ok, sets logged_in = true.
#[tauri::command]
pub fn staff_login(
    auth: State<'_, RwLock<auth::AuthState>>,
    password: String,
) -> Result<(), String> {
    let expected = env::var("STAFF_PASSWORD").map_err(|_| "Missing STAFF_PASSWORD".to_string())?;
    if password == expected {
        let mut st = auth.write().unwrap();
        st.logged_in = true;
        st.last_activity = Some(std::time::Instant::now());
        Ok(())
    } else {
        Err("Invalid password".into())
    }
}

#[tauri::command]
pub fn staff_logout(auth: State<'_, RwLock<crate::auth::AuthState>>) {
    let mut st = auth.write().unwrap();
    st.logged_in = false;
    st.last_activity = None;
}

#[tauri::command]
pub fn check_login_status(auth: State<'_, RwLock<crate::auth::AuthState>>) -> bool {
    let mut st = auth.write().unwrap();
    if let (true, Some(last)) = (st.logged_in, st.last_activity) {
        let elapsed = last.elapsed();
        if elapsed.as_secs() > 60 * 15 {
            // Timeout after 15 min
            st.logged_in = false;
            st.last_activity = None;
            return false;
        } else {
            // Refresh activity timestamp
            st.last_activity = Some(std::time::Instant::now());
            return true;
        }
    }
    false
}
