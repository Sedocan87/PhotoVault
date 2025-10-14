use sqlx::migrate::Migrator;

pub mod manager;

pub static MIGRATOR: Migrator = sqlx::migrate!();