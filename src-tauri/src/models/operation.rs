use super::photo::Photo;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    AddPhoto { photo: Photo },
    Move { from: PathBuf, to: PathBuf },
    Delete { path: PathBuf },
    Rename { path: PathBuf, new_name: String },
    CreateAlbum { name: String },
    DeleteAlbum { album_id: i64 },
    AddToAlbum { photo_id: i64, album_id: i64 },
    AddTag { photo_id: i64, tag_name: String },
}
