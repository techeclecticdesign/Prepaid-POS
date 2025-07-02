use crate::common::auth;
use std::env;
use std::sync::RwLock;
use tauri::State;

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
pub fn staff_logout(auth: State<'_, RwLock<crate::common::auth::AuthState>>) {
    let mut st = auth.write().unwrap();
    st.logged_in = false;
    st.last_activity = None;
}

#[tauri::command]
pub fn check_login_status(auth: State<'_, RwLock<crate::common::auth::AuthState>>) -> bool {
    let mut st = auth.write().unwrap();
    if let (true, Some(last)) = (st.logged_in, st.last_activity) {
        let elapsed = last.elapsed();
        if elapsed.as_secs() > 60 * 15 {
            // Timeout after 15 min
            st.logged_in = false;
            st.last_activity = None;
            return false;
        } else {
            return true;
        }
    }
    false
}

#[tauri::command]
pub fn update_activity(
    auth: tauri::State<'_, std::sync::RwLock<crate::common::auth::AuthState>>,
) -> Result<(), String> {
    let mut st = auth.write().unwrap();
    if st.logged_in {
        st.last_activity = Some(std::time::Instant::now());
    }
    Ok(())
}
