pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use commands::{
    add_tag, add_to_album, create_album, delete_photos, get_config, get_photos,
    get_sync_queue_status, move_photos, rename_photo, scan_library, set_drive_paths,
    verify_sync_status,
};
use db::manager::DatabaseManager;
use services::config::{self, CONFIG_FILE_NAME};
use tauri::{async_runtime::Mutex, Manager};

use crate::services::sync_engine::SyncEngine;

// AppState now holds the database manager
pub struct AppState {
    pub sync_engine: Mutex<Option<SyncEngine>>,
}

pub fn run() {
    // Determine config path at startup
    let config_path = config::get_app_config_dir()
        .expect("Failed to get config dir")
        .join(CONFIG_FILE_NAME);

    // Load config at startup
    let initial_config = tauri::async_runtime::block_on(config::load_config_from_path(&config_path))
        .unwrap_or_default();

    let state = AppState {
        sync_engine: Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_drive_paths,
            verify_sync_status,
            move_photos,
            delete_photos,
            rename_photo,
            get_sync_queue_status,
            scan_library,
            get_photos,
            create_album,
            add_to_album,
            add_tag,
        ])
        .setup(move |app| {
            let handle = app.handle();
            let app_state = handle.state::<AppState>();

            // Initialize the DatabaseManager based on the loaded config
            if let (Some(primary_path), Some(backup_path)) = (
                initial_config.primary_drive,
                initial_config.backup_drive,
            ) {
                let sync_engine = tauri::async_runtime::block_on(async {
                    let primary_pool =
                        DatabaseManager::create_pool(&primary_path.join("photovault.db")).await?;
                    let backup_pool =
                        DatabaseManager::create_pool(&backup_path.join("photovault.db")).await?;
                    Ok::<_, anyhow::Error>(SyncEngine::new(primary_pool, Some(backup_pool)))
                });

                if let Ok(engine) = sync_engine {
                    *app_state.sync_engine.blocking_lock() = Some(engine);
                } else {
                    println!("Failed to initialize SyncEngine on startup.");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}