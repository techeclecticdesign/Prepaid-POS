use crate::common::auth;
use crate::common::rwlock_ext::RwLockExt;
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
        let mut st = auth.safe_write()?;
        st.logged_in = true;
        st.last_activity = Some(std::time::Instant::now());
        Ok(())
    } else {
        Err("Invalid password".into())
    }
}

#[tauri::command]
pub fn staff_logout(auth: State<'_, RwLock<crate::common::auth::AuthState>>) -> Result<(), String> {
    let mut st = auth.safe_write()?;
    st.logged_in = false;
    st.last_activity = None;
    Ok(())
}

#[tauri::command]
pub fn check_login_status(
    auth: State<'_, RwLock<crate::common::auth::AuthState>>,
) -> Result<bool, String> {
    let mut st = auth.safe_write()?;
    if let (true, Some(last)) = (st.logged_in, st.last_activity) {
        let elapsed = last.elapsed();
        if elapsed.as_secs() > 60 * 15 {
            st.logged_in = false;
            st.last_activity = None;
            return Ok(false);
        } else {
            return Ok(true);
        }
    }
    Ok(false)
}

#[tauri::command]
pub fn update_activity(
    auth: tauri::State<'_, std::sync::RwLock<crate::common::auth::AuthState>>,
) -> Result<(), String> {
    let mut st = auth.safe_write()?;
    if st.logged_in {
        st.last_activity = Some(std::time::Instant::now());
    }
    Ok(())
}
