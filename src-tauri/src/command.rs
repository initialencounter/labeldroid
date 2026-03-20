use share::manager::server_manager::SERVER_PORT;

#[cfg(desktop)]
#[tauri::command]
pub fn open_local_dir(target: &str) {
    share::utils::fs::open_local_dir(target);
}

#[tauri::command]
pub fn get_server_port() -> u16 {
    SERVER_PORT.load(std::sync::atomic::Ordering::Relaxed) as u16
}
