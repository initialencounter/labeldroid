use labeldroid_types::config::Config;
use share::manager::server_manager::SERVER_PORT;
#[cfg(desktop)]
use tauri_plugin_autostart::ManagerExt;

#[cfg(desktop)]
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
    #[cfg(desktop)]
    {
        share::utils::fs::open_local_dir(target);
    }
}

// 保存配置, 并重启服务器
#[tauri::command]
pub async fn reload_config(app: tauri::AppHandle, config: Config) -> Result<(), String> {
    #[cfg(desktop)]
    set_auto_start(app.clone(), config.base.auto_start)?;
    Ok(())
}

#[tauri::command]
pub fn get_server_port() -> u16 {
    SERVER_PORT.load(std::sync::atomic::Ordering::Relaxed) as u16
}
