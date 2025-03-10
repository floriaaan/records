use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm, Debug)]
pub struct RecordInput {
    pub title: String,
    pub artist: String,
    pub release_date: String,
    pub cover_url: String,
    pub discogs_url: Option<String>,
    pub spotify_url: Option<String>,
}
