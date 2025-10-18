use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    #[sqlx(default)]
    pub id: i64,
    pub name: String,
}
