use crate::db::manager::DatabaseManager;
use crate::models::operation::Operation;
use crate::services::config::{self, AppConfig, CONFIG_FILE_NAME};
use crate::services::sync_engine::SyncEngine;
use crate::services::sync_status::{self, SyncStatus};
use std::path::PathBuf;
use tauri::{State, async_runtime::Mutex};

pub mod album;
pub mod tag;
pub mod filter;

pub struct AppState {
    pub sync_engine: Mutex<Option<SyncEngine>>,
}

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
pub async fn scan_library(state: State<'_, AppState>) -> CommandResult<()> {
    let mut sync_engine = state.sync_engine.lock().await;
    if let Some(sync_engine) = &mut *sync_engine {
        let config = get_config().await?;
        if let Some(primary_drive) = config.primary_drive {
            sync_engine
                .scan_library(primary_drive.to_str().unwrap())
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_photos(limit: i64, offset: i64, state: State<'_, AppState>) -> CommandResult<Vec<crate::models::photo::Photo>> {
    let sync_engine = state.sync_engine.lock().await;
    if let Some(sync_engine) = &*sync_engine {
        let photos = sync_engine.get_photos(limit, offset).await.map_err(|e| e.to_string())?;
        return Ok(photos);
    }
    Ok(Vec::new())
}

#[tauri::command]
pub async fn move_photos(
    photo_ids: Vec<i64>,
    target_path: String,
    state: State<'_, AppState>,
) -> CommandResult<()> {
    let mut sync_engine = state.sync_engine.lock().await;
    if let Some(sync_engine) = &mut *sync_engine {
        for photo_id in photo_ids {
            let photo = sync_engine.get_photo_by_id(photo_id).await.map_err(|e| e.to_string())?;
            let from = PathBuf::from(&photo.path);
            let to = PathBuf::from(&target_path).join(from.file_name().unwrap());
            tokio::fs::rename(&from, &to).await.map_err(|e| e.to_string())?;
            let op = Operation::Move { from, to };
            sync_engine
                .execute_operation(&op)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[derive(serde::Serialize)]
pub struct QueueStatus {
    pub pending_operations: u32,
}

#[tauri::command]
pub async fn get_sync_queue_status(state: State<'_, AppState>) -> CommandResult<QueueStatus> {
    let sync_engine = state.sync_engine.lock().await;
    if let Some(sync_engine) = &*sync_engine {
        return Ok(QueueStatus {
            pending_operations: sync_engine.queue_len() as u32,
        });
    }
    Ok(QueueStatus {
        pending_operations: 0,
    })
}

#[tauri::command]
pub async fn rename_photo(
    photo_id: i64,
    new_name: String,
    state: State<'_, AppState>,
) -> CommandResult<()> {
    let mut sync_engine = state.sync_engine.lock().await;
    if let Some(sync_engine) = &mut *sync_engine {
        let photo = sync_engine.get_photo_by_id(photo_id).await.map_err(|e| e.to_string())?;
        let from = PathBuf::from(&photo.path);
        let to = from.with_file_name(&new_name);
        tokio::fs::rename(&from, &to).await.map_err(|e| e.to_string())?;
        let op = Operation::Rename { path: from, new_name };
        sync_engine
            .execute_operation(&op)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_photos(photo_ids: Vec<i64>, state: State<'_, AppState>) -> CommandResult<()> {
    let mut sync_engine = state.sync_engine.lock().await;
    if let Some(sync_engine) = &mut *sync_engine {
        for photo_id in photo_ids {
            let photo = sync_engine.get_photo_by_id(photo_id).await.map_err(|e| e.to_string())?;
            let path = PathBuf::from(&photo.path);
            tokio::fs::remove_file(&path).await.map_err(|e| e.to_string())?;
            let op = Operation::Delete { path };
            sync_engine
                .execute_operation(&op)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn set_drive_paths(
    primary: String,
    backup: String,
    state: State<'_, AppState>,
) -> CommandResult<()> {
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
    let primary_pool = DatabaseManager::create_pool(&primary_path.join("photovault.db"))
        .await
        .map_err(|e| e.to_string())?;
    println!("[set_drive_paths] Primary pool created.");

    println!("[set_drive_paths] Creating backup pool...");
    let backup_pool = DatabaseManager::create_pool(&backup_path.join("photovault.db"))
        .await
        .map_err(|e| e.to_string())?;
    println!("[set_drive_paths] Backup pool created.");

    let engine = SyncEngine::new(primary_pool, Some(backup_pool));
    println!("[set_drive_paths] SyncEngine created.");

    let mut sync_engine = state.sync_engine.lock().await;
    *sync_engine = Some(engine);

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