use anyhow::Result;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

pub use crate::models::filter::FilterCriteria;
use crate::models::photo::Photo;

pub async fn filter_photos(pool: &SqlitePool, criteria: FilterCriteria) -> Result<Vec<Photo>> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT p.* FROM photos p ");

    if let Some(tags) = &criteria.tags {
        if !tags.is_empty() {
            query_builder.push("JOIN photo_tags pt ON p.id = pt.photo_id ");
        }
    }

    if let Some(albums) = &criteria.albums {
        if !albums.is_empty() {
            query_builder.push("JOIN album_photos ap ON p.id = ap.photo_id ");
        }
    }

    let mut conditions = Vec::new();

    if let Some(_) = criteria.date_from {
        conditions.push("p.date_taken >= ?".to_string());
    }
    if let Some(_) = criteria.date_to {
        conditions.push("p.date_taken <= ?".to_string());
    }
    if let Some(_) = criteria.min_width {
        conditions.push("p.width >= ?".to_string());
    }
    if let Some(_) = criteria.min_height {
        conditions.push("p.height >= ?".to_string());
    }
    if let Some(query) = &criteria.query {
        if !query.is_empty() {
            conditions.push("p.filename LIKE ?".to_string());
        }
    }
    if let Some(tags) = &criteria.tags {
        if !tags.is_empty() {
            let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            conditions.push(format!("pt.tag_id IN ({})", placeholders));
        }
    }
    if let Some(albums) = &criteria.albums {
        if !albums.is_empty() {
            let placeholders = albums.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            conditions.push(format!("ap.album_id IN ({})", placeholders));
        }
    }

    if !conditions.is_empty() {
        query_builder.push("WHERE ");
        query_builder.push(conditions.join(" AND "));
    }

    let mut query = query_builder.build_query_as();

    if let Some(date_from) = criteria.date_from {
        query = query.bind(date_from);
    }
    if let Some(date_to) = criteria.date_to {
        query = query.bind(date_to);
    }
    if let Some(min_width) = criteria.min_width {
        query = query.bind(min_width);
    }
    if let Some(min_height) = criteria.min_height {
        query = query.bind(min_height);
    }
    if let Some(query_str) = &criteria.query {
        if !query_str.is_empty() {
            query = query.bind(format!("%{}%", query_str));
        }
    }
    if let Some(tags) = &criteria.tags {
        for tag_id in tags {
            query = query.bind(tag_id);
        }
    }
    if let Some(albums) = &criteria.albums {
        for album_id in albums {
            query = query.bind(album_id);
        }
    }

    let photos = query.fetch_all(pool).await?;
    Ok(photos)
}

pub async fn search_photos(pool: &SqlitePool, query: String) -> Result<Vec<Photo>> {
    let criteria = FilterCriteria {
        query: Some(query),
        ..Default::default()
    };
    filter_photos(pool, criteria).await
}
