
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotifyAccessTokenRoot {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotifyRoot {
    pub albums: AlbumsResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlbumsResults {
    pub href: String,
    pub limit: i64,
    pub next: String,
    pub offset: i64,
    pub previous: Value,
    pub total: i64,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "album_type")]
    pub album_type: String,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i64,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub artists: Vec<Artist>,
    #[serde(rename = "is_playable", default)]
    pub is_playable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls2,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalUrls2 {
    pub spotify: String,
}
