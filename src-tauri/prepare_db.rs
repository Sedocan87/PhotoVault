use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = "sqlite:photovault.db";
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        Sqlite::create_database(db_url).await?;
    }

    let db_pool = SqlitePool::connect(db_url).await?;

    let migrator = sqlx::migrate!("./migrations");
    migrator.run(&db_pool).await?;

    Ok(())
}