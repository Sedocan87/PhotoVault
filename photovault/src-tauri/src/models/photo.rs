use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Photo {
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub file_size: Option<i64>,
    pub date_taken: Option<DateTime<Utc>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub format: String,
}