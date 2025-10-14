use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use super::MIGRATOR;

pub struct DatabaseManager {
    pub primary_pool: Pool<Sqlite>,
    pub backup_pool: Option<Pool<Sqlite>>,
}

impl DatabaseManager {
    /// Creates a new DatabaseManager from existing database pools.
    pub fn new(primary_pool: Pool<Sqlite>, backup_pool: Option<Pool<Sqlite>>) -> Self {
        Self {
            primary_pool,
            backup_pool,
        }
    }

    /// Creates a new database pool at the specified path and runs migrations.
    pub async fn create_pool(db_path: &std::path::Path) -> Result<Pool<Sqlite>> {
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent).await?;
            }
        }

        let db_url = format!("sqlite:{}", db_path.to_str().unwrap());
        println!("Connecting to DB at: {}", db_url);
        let options = sqlx::sqlite::SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = sqlx::SqlitePool::connect_with(options).await
            .context(format!("Failed to connect to database at {}", db_url))?;

        MIGRATOR.run(&pool).await.context("Failed to run database migrations")?;

        Ok(pool)
    }
}