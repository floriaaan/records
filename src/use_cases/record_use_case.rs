use std::env;

use crate::db::DbCon;
use crate::dto::discogs_dto::DiscogsRoot;
use crate::dto::record_dto::RecordInput;
use crate::dto::spotify_dto::{SpotifyAccessTokenRoot, SpotifyRoot};
use crate::error::app_error::AppError;
use crate::models::record_model::Record;
use crate::repositories::repositories::Repos;
use chrono::DateTime;
use mockall::automock;
use tracing::instrument;

pub struct RecordUseCaseImpl {}

impl RecordUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait RecordUseCase: Send + Sync {
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
        record: RecordInput,
    ) -> Result<Record, AppError>;
    async fn create_multiple(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
        records: Vec<RecordInput>,
    ) -> Result<Vec<Record>, AppError>;

    async fn find_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<Record>, AppError>;
    async fn find_by_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        id: i32,
    ) -> Result<Option<Record>, AppError>;
    async fn find_all_by_user_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
    ) -> Result<Vec<Record>, AppError>;
    async fn get_random_by_user_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
    ) -> Result<Option<Record>, AppError>;

    async fn search(&self, query: &String) -> Result<Vec<Record>, AppError>;
}

#[async_trait]
impl RecordUseCase for RecordUseCaseImpl {
    #[instrument(name = "record_use_case/create", skip_all)]
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
        record: RecordInput,
    ) -> Result<Record, AppError> {
        let created_record = repos.record.create(&mut *db_con, user_id, record).await?;
        Ok(created_record)
    }

    #[instrument(name = "record_use_case/create_multiple", skip_all)]
    async fn create_multiple(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
        records: Vec<RecordInput>,
    ) -> Result<Vec<Record>, AppError> {
        let created_records = repos.record.create_multiple(&mut *db_con, user_id, records).await?;
        Ok(created_records)
    }

    #[instrument(name = "record_use_case/find_all", skip_all)]
    async fn find_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<Record>, AppError> {
        let records = repos.record.find_all(&mut *db_con).await?;
        Ok(records)
    }

    #[instrument(name = "record_use_case/find_all_by_user_id", skip_all)]
    async fn find_all_by_user_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
    ) -> Result<Vec<Record>, AppError> {
        let records = repos
            .record
            .find_all_by_user_id(&mut *db_con, user_id)
            .await?;
        Ok(records)
    }

    #[instrument(name = "record_use_case/get_random_by_user_id", skip_all)]
    async fn get_random_by_user_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        user_id: i32,
    ) -> Result<Option<Record>, AppError> {
        let record = repos
            .record
            .get_random_by_user_id(&mut *db_con, user_id)
            .await?;
        Ok(record)
    }

    #[instrument(name = "record_use_case/find_by_id", skip_all)]
    async fn find_by_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        id: i32,
    ) -> Result<Option<Record>, AppError> {
        let record = repos.record.find_by_id(&mut *db_con, id).await?;
        Ok(record)
    }

    #[instrument(name = "record_use_case/search", skip_all)]
    async fn search(&self, query: &String) -> Result<Vec<Record>, AppError> {
        // Get records result from discogs
        let discogs_secret = env::var("DISCOGS_SECRET").expect("DISCOGS_SECRET must be set.");
        let spotify_client_id =
            env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set.");
        let spotify_client_secret =
            env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set.");
        let spotify_refresh_token =
            env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set.");

        // Authenticate with spotify
        let spotify_access_token: String = match reqwest::Client::new()
            .post("https://accounts.spotify.com/api/token")
            .header(
                "Authorization",
                format!("Basic {}:{}", spotify_client_id, spotify_client_secret),
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type=refresh_token&refresh_token={}",
                spotify_refresh_token
            ))
            .send()
            .await
        {
            Ok(result) => match result.json::<SpotifyAccessTokenRoot>().await {
                Ok(json) => json.access_token,
                Err(e) => {
                    println!("[SpotifyAccessTokenRoot.json] Error: {}", e);
                    return Err(AppError::new(500, &e.to_string()));
                }
            },
            Err(e) => {
                println!("[SpotifyAccessTokenRoot.send] Error: {}", e);
                return Err(AppError::new(500, &e.to_string()));
            }
        };

        let spotify_json: Option<SpotifyRoot> = match reqwest::Client::new()
            .get("https://api.spotify.com/v1/search")
            .query(&[("q", query)])
            .query(&[("type", "album")])
            .header("Authorization", format!("Bearer {}", spotify_access_token))
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(result) => match result.json::<SpotifyRoot>().await {
                Ok(result) => Some(result),
                Err(e) => {
                    println!("[SpotifyRoot.json] Error: {}", e);
                    return Err(AppError::new(500, &e.to_string()));
                }
            },
            Err(e) => {
                println!("[SpotifyRoot.send] Error: {}", e);
                return Err(AppError::new(500, &e.to_string()));
            }
        };

        let discogs_json = match reqwest::Client::new()
            .get("https://api.discogs.com/database/search")
            .query(&[("q", query)])
            .query(&[("type", "master")])
            .header("Authorization", format!("Discogs token={}", discogs_secret))
            .header("User-Agent", "vinyl-api")
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(result) => match result.json::<DiscogsRoot>().await {
                Ok(result) => result.results,
                Err(e) => return Err(AppError::new(500, &e.to_string())),
            },
            Err(e) => return Err(AppError::new(500, &e.to_string())),
        };

        let records = discogs_json
            .iter()
            .map(|record| {
                let release_date = DateTime::parse_from_rfc3339(
                    format!(
                        "{}-01-01T00:00:00Z",
                        record.year.as_ref().unwrap_or(&"0000".to_string())
                    )
                    .as_str(),
                )
                .unwrap_or_default()
                .date_naive();

                Record {
                    id: 0,
                    user_id: 0,
                    title: record.title.clone(),
                    artist: record.title.clone().split(" - ").collect::<Vec<&str>>()[0].to_string(),
                    release_date: release_date, // TODO: get release day from discogs - or spotify
                    cover_url: record.cover_image.clone(), // TODO: get cover image from spotify
                    discogs_url: Some(record.master_url.clone()),
                    spotify_url: None, // TODO: get spotify url
                }
            })
            .collect();

        // Increment results via spotify search results

        let spotify_json = match spotify_json {
            Some(spotify_json) => spotify_json,
            None => return Ok(records),
        };

        let records = records
            .iter()
            .map(|record| {
                let spotify_record = spotify_json
                    .albums
                    .items
                    .iter()
                    .find(|item| item.name == record.title)
                    .unwrap_or(&spotify_json.albums.items[0]);

                let release_date =
                    DateTime::parse_from_rfc3339(spotify_record.release_date.as_str())
                        .unwrap_or_default()
                        .date_naive();

                Record {
                    id: 0,
                    user_id: 0,
                    title: record.title.clone(),
                    artist: record.artist.clone(),
                    release_date: release_date,
                    cover_url: spotify_record.images[0].url.clone(),
                    discogs_url: record.discogs_url.clone(),
                    spotify_url: Some(spotify_record.external_urls.spotify.clone()),
                }
            })
            .collect();

        Ok(records)
    }
}
