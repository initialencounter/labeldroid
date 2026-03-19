use labeldroid_types::config::Config;
use share::manager::server_manager::SERVER_PORT;
use std::path::Path;
use tauri::Manager;
use tauri_plugin_autostart::ManagerExt;

pub fn set_auto_start(app: tauri::AppHandle, auto_start: bool) -> Result<(), String> {
    let autostart_manager = app.autolaunch();
    if auto_start {
        let _ = autostart_manager.enable();
    } else {
        let _ = autostart_manager.disable();
    }
    Ok(())
}

#[tauri::command]
pub fn open_local_dir(target: &str) {
    share::utils::fs::open_local_dir(target);
}

#[tauri::command]
pub fn open_with_wps(target: &str, name: &str) {
    let file_path = Path::new(target).join(Path::new(name));
    let _ = std::process::Command::new("wps").arg(file_path).spawn();
}

#[tauri::command]
pub fn minimize_window(app: tauri::AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.minimize().unwrap();
}

#[tauri::command]
pub fn hide_window(app: tauri::AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.hide().unwrap();
}

#[tauri::command]
pub fn maximize_window(app: tauri::AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.maximize().unwrap();
}

#[tauri::command]
pub fn unmaximize_window(app: tauri::AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.unmaximize().unwrap();
}

// 保存配置, 并重启服务器
#[tauri::command]
pub async fn reload_config(app: tauri::AppHandle, config: Config) -> Result<(), String> {
    set_auto_start(app.clone(), config.base.auto_start)?;
    Ok(())
}

#[tauri::command]
pub fn get_server_port() -> u16 {
    SERVER_PORT.load(std::sync::atomic::Ordering::Relaxed) as u16
}
