pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use db::manager::DatabaseManager;
use services::config::{self, AppConfig, CONFIG_FILE_NAME};
use std::sync::Mutex;
use tauri::Manager;

// Import the new commands
use commands::{get_config, set_drive_paths, verify_sync_status};

// AppState now holds the config and the database manager
pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub db_manager: Mutex<Option<DatabaseManager>>,
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
        config: Mutex::new(initial_config.clone()),
        db_manager: Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_config,
            set_drive_paths,
            verify_sync_status,
        ])
        .setup(move |app| {
            let handle = app.handle();
            let app_state = handle.state::<AppState>();

            // Initialize the DatabaseManager based on the loaded config
            if let (Some(primary_path), Some(backup_path)) = (
                initial_config.primary_drive,
                initial_config.backup_drive,
            ) {
                let db_manager = tauri::async_runtime::block_on(async {
                    let primary_pool = DatabaseManager::create_pool(&primary_path.join("photovault.db")).await?;
                    let backup_pool = DatabaseManager::create_pool(&backup_path.join("photovault.db")).await?;
                    Ok::<_, anyhow::Error>(DatabaseManager::new(primary_pool, Some(backup_pool)))
                });

                if let Ok(manager) = db_manager {
                    *app_state.db_manager.lock().unwrap() = Some(manager);
                } else {
                    println!("Failed to initialize DatabaseManager on startup.");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}