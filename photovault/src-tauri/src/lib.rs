use commands::{
    delete_photos, get_config, get_photos, move_photos, rename_photo, scan_library,
    set_drive_paths, verify_sync_status_command, AppState,
};
use db::manager::DatabaseManager;
use services::config::AppConfig;
use services::sync_engine::SyncEngine;
use tokio::sync::Mutex;

pub mod models;
pub mod services;
pub mod commands;
pub mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_config = AppConfig::load().expect("Failed to load app config");
    let db_manager = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(DatabaseManager::new(
            app_config.primary_drive,
            app_config.backup_drive,
        ))
        .expect("Failed to initialize database manager");

    let sync_engine = SyncEngine::new(db_manager);
    let app_state = AppState {
        sync_engine: Mutex::new(sync_engine),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_library,
            get_photos,
            set_drive_paths,
            get_config,
            verify_sync_status_command,
            move_photos,
            delete_photos,
            rename_photo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    mod file_ops;
    mod db;
    mod sync_engine;
}
