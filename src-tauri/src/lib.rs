mod apply;
mod command;
mod handle;
mod menu;
mod utils;
use crate::command as cmd;

#[cfg(desktop)]
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{Target, TargetKind};

use crate::handle::handle_setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(desktop)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init());

    #[cfg(not(desktop))]
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init());

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ));
    }

    builder.plugin(
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
        .setup(|app| {
            #[cfg(desktop)]
            handle_setup(app);
            apply::apply(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::reload_config,
            cmd::open_local_dir,
            #[cfg(desktop)]
            cmd::minimize_window,
            #[cfg(desktop)]
            cmd::maximize_window,
            #[cfg(desktop)]
            cmd::unmaximize_window,
            #[cfg(desktop)]
            cmd::hide_window,
            cmd::get_server_port,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
