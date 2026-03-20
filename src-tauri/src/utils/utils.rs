#[cfg(desktop)]
use reqwest;
#[cfg(desktop)]
use serde::Deserialize;
#[cfg(desktop)]
use tauri::{self, WebviewWindow};

#[cfg(desktop)]
#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[cfg(desktop)]
#[tauri::command]
pub fn restart() {
    tauri::process::restart(&tauri::Env::default())
}

#[cfg(desktop)]
pub fn check_update(flag: String) -> String {
    let url = "https://api.github.com/repos/initialencounter/labeldroid/releases/latest";
    let client = reqwest::blocking::Client::new();

    let resp = match client
        .get(url)
        .header(reqwest::header::USER_AGENT, "rust-app")
        .send()
    {
        Ok(response) => response,
        Err(_) => return flag,
    };

    if !resp.status().is_success() {
        return flag;
    }

    let release = match resp.json::<Release>() {
        Ok(release) => release,
        Err(_) => return flag,
    };
    release.tag_name
}

#[cfg(desktop)]
pub fn hide_or_show(window: WebviewWindow) {
    if window.is_visible().unwrap_or_default() {
        if window.is_minimized().unwrap_or_default() {
            let _ = window.unminimize();
            let _ = window.set_focus();
            let _ = window.set_always_on_top(true);
        } else {
            let _ = window.hide();
        }
    } else {
        let _ = window.show();
        let _ = window.set_focus();
        let _ = window.set_always_on_top(true);
    }
}
