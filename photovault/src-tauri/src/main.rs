#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod db;
mod services;

use commands::photo_commands::{
    AppState,
    delete_photos, get_sync_queue_status, move_photos, rename_photo,
};
use sync_logic::SyncEngine;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create pool.");

    let sync_engine = SyncEngine::new(pool, None, "/tmp/primary".into(), Some("/tmp/backup".into())).await;

    tauri::Builder::default()
        .manage(AppState {
            sync_engine: Mutex::new(sync_engine),
        })
        .invoke_handler(tauri::generate_handler![
            move_photos,
            delete_photos,
            rename_photo,
            get_sync_queue_status,
            flush_sync_queue
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    mod sync_engine_tests;
}