use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

pub struct DatabaseManager {
    pub primary_db: SqlitePool,
    pub backup_db: Option<SqlitePool>,
}

impl DatabaseManager {
    pub async fn new(primary_path: PathBuf, backup_path: Option<PathBuf>) -> Result<Self, sqlx::Error> {
        if !Sqlite::database_exists(primary_path.to_str().unwrap()).await? {
            Sqlite::create_database(primary_path.to_str().unwrap()).await?;
        }
        let primary_db = SqlitePool::connect(primary_path.to_str().unwrap()).await?;
        sqlx::migrate!("./migrations").run(&primary_db).await?;

        let backup_db = if let Some(backup_path) = backup_path {
            if !Sqlite::database_exists(backup_path.to_str().unwrap()).await.unwrap_or(false) {
                Sqlite::create_database(backup_path.to_str().unwrap()).await?;
            }
            let pool = SqlitePool::connect(backup_path.to_str().unwrap()).await?;
            sqlx::migrate!("./migrations").run(&pool).await?;
            Some(pool)
        } else {
            None
        };

        Ok(Self {
            primary_db,
            backup_db,
        })
    }

    pub async fn execute_on_both<F>(&mut self, query_fn: F) -> Result<(), sqlx::Error>
    where
        F: for<'c> Fn(&'c SqlitePool) -> Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send + 'c>>,
    {
        query_fn(&self.primary_db).await?;
        if let Some(backup_db) = &self.backup_db {
            if backup_db.is_closed() {
                return Err(sqlx::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Backup database is disconnected",
                )));
            }
            query_fn(backup_db).await?;
        }
        Ok(())
    }
}