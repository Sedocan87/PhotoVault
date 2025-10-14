use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub file_size: u64,
    pub date_taken: Option<DateTime<Utc>>,
    pub width: u32,
    pub height: u32,
    pub format: String,
}