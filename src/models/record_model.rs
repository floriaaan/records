use crate::models::tag_model::{Tag, TagResponse};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// RecordDB is used only for database operations and doesn't include the tags field
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RecordDB {
    pub id: i32,
    
    pub title: String,
    pub artist: String,
    pub release_date: chrono::NaiveDate,
    pub cover_url: String,

    pub discogs_url: Option<String>,
    pub spotify_url: Option<String>,

    pub owned: bool,
    pub wanted: bool,

    pub user_id: i32,
}

/// Record is the complete model including tags
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: i32,
    
    pub title: String,
    pub artist: String,
    pub release_date: chrono::NaiveDate,
    pub cover_url: String,

    pub discogs_url: Option<String>,
    pub spotify_url: Option<String>,

    pub owned: bool,
    pub wanted: bool,

    pub user_id: i32,

    /// Tags associated with this record
    /// This field is not stored in the database
    /// but is populated after retrieval
    /// from the database using a join query.
    /// Uses TagResponse to avoid exposing internal IDs
    pub tags: Option<Vec<TagResponse>>,
}

impl From<RecordDB> for Record {
    fn from(db: RecordDB) -> Self {
        Self {
            id: db.id,
            title: db.title,
            artist: db.artist,
            release_date: db.release_date,
            cover_url: db.cover_url,
            discogs_url: db.discogs_url,
            spotify_url: db.spotify_url,
            owned: db.owned,
            wanted: db.wanted,
            user_id: db.user_id,
            tags: None,
        }
    }
}

impl Record {
    // Helper method to add tags to a record after database retrieval
    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        // Convert Tag to TagResponse to hide IDs
        self.tags = Some(tags.into_iter().map(TagResponse::from).collect());
        self
    }
}
