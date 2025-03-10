use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    
    pub title: String,
    pub artist: String,
    pub release_date: chrono::NaiveDate,
    pub cover_url: String,

    pub discogs_url: Option<String>,
    pub spotify_url: Option<String>,

    pub user_id: i32,
}
