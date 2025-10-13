pub mod commands;
pub mod db;
pub mod models;
pub mod services;

use sqlx::SqlitePool;
use std::sync::Mutex;
use tauri::Manager;
use commands::{get_photos, scan_library};


pub struct AppState {
    pub db_pool: Mutex<Option<SqlitePool>>,
}

pub fn run() {
    let state = AppState {
        db_pool: std::sync::Mutex::new(None),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![scan_library, get_photos])
        .setup(|app| {
            let handle = app.handle();
            let app_state = handle.state::<AppState>();
            let app_dir = handle.path().app_data_dir().unwrap();
            std::fs::create_dir_all(&app_dir).unwrap();
            let db_path = app_dir.join("photovault.db");

            // Initialize the database
            let db_pool = tauri::async_runtime::block_on(db::init_db(db_path.to_str().unwrap())).unwrap();
            *app_state.db_pool.lock().unwrap() = Some(db_pool);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}