use crate::models::photo::Photo;
use crate::services::file_ops::FileOperationService;
use crate::AppState;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use tauri::State;

#[derive(FromRow)]
struct DbPhoto {
    id: i64,
    path: Option<String>,
    filename: Option<String>,
    file_size: Option<i64>,
    date_taken: Option<NaiveDateTime>,
    width: Option<i64>,
    height: Option<i64>,
    format: Option<String>,
}

impl From<DbPhoto> for Photo {
    fn from(db_photo: DbPhoto) -> Self {
        Photo {
            id: db_photo.id,
            path: db_photo.path.unwrap_or_default(),
            filename: db_photo.filename.unwrap_or_default(),
            file_size: db_photo.file_size,
            date_taken: db_photo.date_taken.map(|ndt| DateTime::from_naive_utc_and_offset(ndt, Utc)),
            width: db_photo.width,
            height: db_photo.height,
            format: db_photo.format.unwrap_or_default(),
        }
    }
}

#[tauri::command]
pub async fn scan_library(primary_path: String, state: State<'_, AppState>) -> Result<Vec<Photo>, String> {
    let service = FileOperationService::new(primary_path);
    let photos = service.scan_directory().await?;

    let db_pool = state.db_pool.lock().unwrap().clone().unwrap();

    for photo in &photos {
        sqlx::query!(
            r#"
            INSERT OR IGNORE INTO photos (path, filename, file_hash, file_size, date_taken, width, height, format)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            photo.path,
            photo.filename,
            "", // Placeholder for hash
            photo.file_size,
            photo.date_taken,
            photo.width,
            photo.height,
            photo.format,
        )
        .execute(&db_pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(photos)
}

#[tauri::command]
pub async fn get_photos(limit: i64, offset: i64, state: State<'_, AppState>) -> Result<Vec<Photo>, String> {
    let db_pool = state.db_pool.lock().unwrap().clone().unwrap();

    let db_photos = sqlx::query_as!(
        DbPhoto,
        r#"
        SELECT id, path, filename, file_size, date_taken, width, height, format
        FROM photos
        ORDER BY date_added DESC
        LIMIT ?1 OFFSET ?2
        "#,
        limit,
        offset
    )
    .fetch_all(&db_pool)
    .await
    .map_err(|e| e.to_string())?;

    let photos = db_photos.into_iter().map(Photo::from).collect();

    Ok(photos)
}