use share::config::ConfigManager;
use share::manager::server_manager::ServerManager;
use std::path::PathBuf;
use tauri::{App, Manager};

pub fn apply(app: &mut App) {
    let workspace_dir = if let Ok(dir) = app.app_handle().path().download_dir() {
        dir
    } else {
        #[cfg(target_os = "android")]
        let fallback_dir = PathBuf::from("/storage/emulated/0/Download/labeldroid");

        #[cfg(not(target_os = "android"))]
        let fallback_dir = {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            current_dir.parent().unwrap_or(&current_dir).join("tmp")
        };

        fallback_dir
    };

    if !workspace_dir.exists() {
        std::fs::create_dir_all(&workspace_dir).unwrap_or(());
    }
    let config = ConfigManager::get_config();
    let server_manager = ServerManager::new(workspace_dir, config.server.port);
    server_manager.start();
    app.manage(server_manager);
}
