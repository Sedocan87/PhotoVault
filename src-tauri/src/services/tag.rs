use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::tag::Tag;

pub struct TagService;

impl TagService {
    pub async fn add_tag(pool: &SqlitePool, photo_id: i64, tag_name: String) -> Result<()> {
        // Find or create the tag
        let tag_id = sqlx::query!(
        "INSERT INTO tags (name) VALUES (?) ON CONFLICT(name) DO UPDATE SET name=name RETURNING id",
        tag_name
    )
        .fetch_one(pool)
        .await?
        .id;

        // Associate tag with photo
        sqlx::query!(
            "INSERT OR IGNORE INTO photo_tags (photo_id, tag_id) VALUES (?, ?)",
            photo_id,
            tag_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn remove_tag(pool: &SqlitePool, photo_id: i64, tag_id: i64) -> Result<()> {
        // Remove the association
        sqlx::query!(
            "DELETE FROM photo_tags WHERE photo_id = ? AND tag_id = ?",
            photo_id,
            tag_id
        )
        .execute(pool)
        .await?;

        // Check if the tag is still associated with any other photos
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM photo_tags WHERE tag_id = ?", tag_id)
            .fetch_one(pool)
            .await?;

        // If the tag is no longer associated with any photos, delete it
        if count == 0 {
            sqlx::query!("DELETE FROM tags WHERE id = ?", tag_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn get_photo_tags(pool: &SqlitePool, photo_id: i64) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as!(
            Tag,
            "SELECT t.id, t.name
         FROM tags t
         JOIN photo_tags pt ON t.id = pt.tag_id
         WHERE pt.photo_id = ?",
            photo_id
        )
        .fetch_all(pool)
        .await?;

        Ok(tags)
    }

    pub async fn get_all_tags(pool: &SqlitePool) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as!(Tag, "SELECT id, name FROM tags")
            .fetch_all(pool)
            .await?;
        Ok(tags)
    }
}
