use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, FromForm, Debug, Validate)]
pub struct RecordInput {
    #[validate(length(min = 1))]
    pub title: String,

    #[validate(length(min = 1))]
    pub artist: String,

    #[validate(length(min = 8))]
    pub release_date: String,

    #[validate(url)]
    pub cover_url: String,

    #[validate(url)]
    pub discogs_url: Option<String>,

    #[validate(url)]
    pub spotify_url: Option<String>,
}
