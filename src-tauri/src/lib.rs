mod server;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info) // 只输出 Info 及以上级别，过滤 Debug
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                // 可以按模块过滤
                .filter(|metadata| {
                    // 忽略某些crate的debug日志
                    !metadata.target().starts_with("hyper::")
                        && !metadata.target().starts_with("tauri::")
                        && !metadata.target().contains("_app.labeldroid:")
                })
                .build(),
        )
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let server = server::FileManangerServer::new(app.handle().clone());
            let _ = server.start();
            app.manage(server);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
