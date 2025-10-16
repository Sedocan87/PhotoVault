use photovault::services::album::AlbumService;
use photovault::services::sync_engine::SyncEngine;
use photovault::db::manager::DatabaseManager;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::pool::Pool;
use sqlx::Sqlite;
use std::path::Path;

async fn setup_test_db(db_name: &str) -> Pool<Sqlite> {
    let db_path = Path::new(db_name);
    if db_path.exists() {
        std::fs::remove_file(db_path).unwrap();
    }
    let pool = DatabaseManager::create_pool(db_path).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_create_album() {
    let pool = setup_test_db("test_create_album.db").await;
    let sync_engine = Arc::new(Mutex::new(Some(SyncEngine::new(pool, None))));
    let album_service = AlbumService::new(sync_engine.clone());

    let album_name = "Test Album".to_string();
    album_service.create_album(album_name.clone()).await.unwrap();

    let albums = album_service.get_albums().await.unwrap();
    assert_eq!(albums.len(), 1);
    assert_eq!(albums[0].name, album_name);
}

use std::fs;

#[tokio::test]
async fn test_add_photos_to_album() {
    let pool = setup_test_db("test_add_photos_to_album.db").await;
    let sync_engine = Arc::new(Mutex::new(Some(SyncEngine::new(pool, None))));
    let album_service = AlbumService::new(sync_engine.clone());

    // Create an album
    let album_name = "Test Album".to_string();
    album_service.create_album(album_name.clone()).await.unwrap();
    let albums = album_service.get_albums().await.unwrap();
    let album_id = albums[0].id;

    // Create a dummy file
    let photo_path = Path::new("test_photo.jpg");
    fs::write(photo_path, "test").unwrap();

    // Create a photo
    let photo = photovault::models::photo::Photo {
        id: 0,
        path: photo_path.to_str().unwrap().to_string(),
        filename: "photo.jpg".to_string(),
        file_size: Some(100),
        date_taken: None,
        width: Some(100),
        height: Some(100),
        format: "jpeg".to_string(),
    };
    {
        let mut sync_engine_locked = sync_engine.lock().await;
        if let Some(engine) = sync_engine_locked.as_mut() {
            engine.add_photo(photo).await.unwrap();
        }
    }

    let photos = {
        let mut sync_engine_locked = sync_engine.lock().await;
        if let Some(engine) = sync_engine_locked.as_mut() {
            engine.get_photos(1, 0).await.unwrap()
        } else {
            vec![]
        }
    };
    let photo_id = photos[0].id;

    // Add photo to album
    album_service.add_photos_to_album(vec![photo_id], album_id).await.unwrap();

    // Verify
    let sync_engine_locked = sync_engine.lock().await;
    if let Some(engine) = sync_engine_locked.as_ref() {
        let photo_albums: Vec<(i64, i64)> = sqlx::query_as("SELECT photo_id, album_id FROM photo_albums WHERE album_id = ? AND photo_id = ?")
            .bind(album_id)
            .bind(photo_id)
            .fetch_all(&engine.primary_db)
            .await
            .unwrap();
        assert_eq!(photo_albums.len(), 1);
    } else {
        panic!("Sync engine not initialized");
    }

    // Clean up dummy file
    fs::remove_file(photo_path).unwrap();
}

#[tokio::test]
async fn test_delete_album() {
    let pool = setup_test_db("test_delete_album.db").await;
    let sync_engine = Arc::new(Mutex::new(Some(SyncEngine::new(pool, None))));
    let album_service = AlbumService::new(sync_engine.clone());

    // Create an album
    let album_name = "Test Album".to_string();
    album_service.create_album(album_name.clone()).await.unwrap();
    let albums = album_service.get_albums().await.unwrap();
    assert_eq!(albums.len(), 1);
    let album_id = albums[0].id;

    // Delete the album
    album_service.delete_album(album_id).await.unwrap();

    // Verify
    let albums = album_service.get_albums().await.unwrap();
    assert_eq!(albums.len(), 0);
}