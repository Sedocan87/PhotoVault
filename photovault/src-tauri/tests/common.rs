use photovault::db::manager::DatabaseManager;
use sqlx::{Pool, Sqlite};

/// Creates a new, anonymous, in-memory database pool for testing.
pub async fn create_in_memory_db_pool() -> Pool<Sqlite> {
    // The path ":memory:" tells SQLite to create a temporary, private, in-memory database.
    let db_path = std::path::Path::new(":memory:");
    DatabaseManager::create_pool(db_path).await.unwrap()
}