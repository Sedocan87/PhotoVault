use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::path::Path;

pub async fn init_db(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
        Sqlite::create_database(db_path).await?;
    }

    let db_pool = SqlitePool::connect(db_path).await?;

    let migrator = sqlx::migrate!("./migrations");
    migrator.run(&db_pool).await?;

    Ok(db_pool)
}