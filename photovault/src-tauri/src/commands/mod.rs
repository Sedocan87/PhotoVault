use crate::db::manager::DatabaseManager;
use crate::services::config::{self, AppConfig, CONFIG_FILE_NAME};
use crate::services::sync_status::{self, SyncStatus};
use std::path::PathBuf;

// The `Result` type alias is not used in this file, but it's good practice to have it.
#[allow(dead_code)]
type CommandResult<T> = Result<T, String>;

#[tauri::command]
pub async fn get_config() -> CommandResult<AppConfig> {
    let config_dir = config::get_app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    config::load_config_from_path(&config_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_drive_paths(primary: String, backup: String) -> CommandResult<()> {
    println!("[set_drive_paths] Command started.");
    let config_dir = config::get_app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    println!("[set_drive_paths] Config path: {:?}", config_path);

    let mut current_config = config::load_config_from_path(&config_path)
        .await
        .map_err(|e| e.to_string())?;
    println!("[set_drive_paths] Config loaded.");

    let primary_path = PathBuf::from(primary);
    let backup_path = PathBuf::from(backup);

    if !primary_path.is_dir() {
        return Err("Primary path is not a valid directory.".to_string());
    }
    if !backup_path.is_dir() {
        return Err("Backup path is not a valid directory.".to_string());
    }

    current_config.primary_drive = Some(primary_path.clone());
    current_config.backup_drive = Some(backup_path.clone());

    config::save_config_to_path(&current_config, &config_path)
        .await
        .map_err(|e| e.to_string())?;
    println!("[set_drive_paths] Config saved.");

    println!("[set_drive_paths] Creating primary pool...");
    let primary_pool = DatabaseManager::create_pool(&primary_path.join("photovault.db")).await.map_err(|e| e.to_string())?;
    println!("[set_drive_paths] Primary pool created.");

    println!("[set_drive_paths] Creating backup pool...");
    let backup_pool = DatabaseManager::create_pool(&backup_path.join("photovault.db")).await.map_err(|e| e.to_string())?;
    println!("[set_drive_paths] Backup pool created.");

    let _manager = DatabaseManager::new(primary_pool, Some(backup_pool));
    println!("[set_drive_paths] DatabaseManager created.");

    Ok(())
}

#[tauri::command]
pub async fn verify_sync_status() -> CommandResult<SyncStatus> {
    let config_dir = config::get_app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    let config = config::load_config_from_path(&config_path)
        .await
        .map_err(|e| e.to_string())?;

    sync_status::verify_sync_status(&config)
        .await
        .map_err(|e| e.to_string())
}