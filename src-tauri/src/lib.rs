pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use commands::album::{
    add_photos_to_album, create_album, delete_album, get_albums, get_photos_by_album,
};
use commands::duplicates::{delete_duplicates, find_duplicates};
use commands::filter::{filter_photos_command, search_photos_command};
use commands::tag::{add_tag, get_all_tags};
use commands::{
    delete_photos, get_config, get_photos, get_sync_queue_status, move_photos, rename_photo,
    scan_library, set_drive_paths, verify_sync_status, AppState,
};
use db::manager::DatabaseManager;
use services::config::{self, CONFIG_FILE_NAME};
use tauri::{async_runtime::Mutex, Manager};

use crate::services::sync_engine::SyncEngine;

pub fn run() {
    // Determine config path at startup
    let config_path = config::get_app_config_dir()
        .expect("Failed to get config dir")
        .join(CONFIG_FILE_NAME);

    // Load config at startup
    let initial_config =
        tauri::async_runtime::block_on(config::load_config_from_path(&config_path))
            .unwrap_or_default();

    let state = AppState {
        db_pool: Mutex::new(None),
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
            add_photos_to_album,
            get_albums,
            delete_album,
            get_photos_by_album,
            add_tag,
            get_all_tags,
            filter_photos_command,
            search_photos_command,
            find_duplicates,
            delete_duplicates,
        ])
        .setup(move |app| {
            let handle = app.handle();
            let app_state = handle.state::<AppState>();

            // Initialize the DatabaseManager based on the loaded config
            if let (Some(primary_path), Some(backup_path)) =
                (initial_config.primary_drive, initial_config.backup_drive)
            {
                let result = tauri::async_runtime::block_on(async {
                    let primary_pool =
                        DatabaseManager::create_pool(&primary_path.join("photovault.db")).await?;
                    let backup_pool =
                        DatabaseManager::create_pool(&backup_path.join("photovault.db")).await?;
                    let engine = SyncEngine::new(primary_pool.clone(), Some(backup_pool));
                    Ok::<_, anyhow::Error>((primary_pool, engine))
                });

                if let Ok((pool, engine)) = result {
                    *app_state.db_pool.blocking_lock() = Some(pool);
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
