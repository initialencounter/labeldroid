use share::manager::server_manager::ServerManager;
use std::path::PathBuf;
use tauri::{App, Manager};

pub fn apply(app: &mut App) {
    let workspace_dir = if let Ok(dir) = app.app_handle().path().document_dir() {
        #[cfg(desktop)]
        let dir = dir.join("labeldroid").join("datasets");
        dir
    } else {
        #[cfg(target_os = "android")]
        let fallback_dir = PathBuf::from("/storage/emulated/0/Android/data/com.initialencounter.labeldroid/files/Documents");

        #[cfg(not(target_os = "android"))]
        let fallback_dir = {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            current_dir.parent().unwrap_or(&current_dir).join("tmp")
        };

        fallback_dir
    };
    // log::info!("Using workspace directory: {:?}", workspace_dir);
    if !workspace_dir.exists() {
        std::fs::create_dir_all(&workspace_dir).unwrap_or(());
    }
    let server_manager = ServerManager::new(workspace_dir, 3480);
    server_manager.start();
    app.manage(server_manager);
}
