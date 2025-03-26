use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Deserialize, Serialize, FromForm, Debug, Validate, JsonSchema)]
pub struct RecordInput {
    #[validate(length(min = 1, message = "Title is required"))]
    #[schemars(example = "Discovery")]
    pub title: String,

    #[validate(length(min = 1, message = "Artist is required"))]
    #[schemars(example = "Daft Punk")]
    pub artist: String,

    #[validate(length(
        min = 8,
        message = "Release date must be at least 8 characters long (e.g. 2025-01-01)"
    ))]
    #[schemars(example = "2001-03-12")]
    pub release_date: String,

    #[validate(url(message = "Cover URL is not a valid URL"))]
    #[schemars(example = "https://upload.wikimedia.org/wikipedia/en/2/27/Daft_Punk_-_Discovery.png")]
    pub cover_url: String,

    #[validate(url(message = "Discogs URL is not a valid URL"))]
    #[schemars(example = "https://www.discogs.com/fr/master/10367-Daft-Punk-Discovery")]
    pub discogs_url: Option<String>,

    #[validate(url(message = "Spotify URL is not a valid URL"))]
    #[schemars(example = "https://open.spotify.com/album/2noRn2Aes5aoNVsU6iWThc")]
    pub spotify_url: Option<String>,
}
