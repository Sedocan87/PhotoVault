use anyhow::Result;
use chrono::{DateTime, Utc};
use image::{GenericImageView, ImageFormat};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Photo {
    #[sqlx(default)]
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub file_hash: Option<String>,
    pub file_size: Option<i64>,
    pub date_taken: Option<DateTime<Utc>>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub format: String,
}

impl Photo {
    pub fn new_from_path(path: PathBuf) -> Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let file_size = metadata.len() as i64;

        let img = image::open(&path)?;
        let (width, height) = img.dimensions();
        let format = format!("{:?}", ImageFormat::from_path(&path)?);

        Ok(Self {
            id: 0,
            path: path.to_str().unwrap().to_string(),
            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
            file_hash: None,
            file_size: Some(file_size),
            date_taken: None, // TODO: Read from EXIF
            width: Some(width as i64),
            height: Some(height as i64),
            format,
        })
    }
}
