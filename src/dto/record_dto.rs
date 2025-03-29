use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct RecordInput {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,

    #[validate(length(min = 1, message = "Artist is required"))]
    pub artist: String,

    #[validate(length(
        min = 8,
        message = "Release date must be at least 8 characters long (e.g. 2025-01-01)"
    ))]
    pub release_date: String,

    #[validate(url(message = "Cover URL is not a valid URL"))]
    pub cover_url: String,

    #[validate(url(message = "Discogs URL is not a valid URL"))]
    pub discogs_url: Option<String>,

    #[validate(url(message = "Spotify URL is not a valid URL"))]
    pub spotify_url: Option<String>,

    pub owned: Option<bool>,

    pub wanted: Option<bool>,

    /// Tags associated with this record (tag names)
    pub tags: Option<Vec<String>>,
}
