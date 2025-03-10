use crate::db::DbCon;
use crate::error::app_error::AppError;
use crate::models::record_model::Record;
use crate::repositories::repositories::Repos;
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
        title: &String,
        artist: &String,
        release_date: &String,
        cover_url: &String,
        discogs_url: Option<String>,
        spotify_url: Option<String>,
    ) -> Result<Record, AppError>;
    async fn find_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<Record>, AppError>;
    async fn find_by_id(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        id: i32,
    ) -> Result<Option<Record>, AppError>;
}

#[async_trait]
impl RecordUseCase for RecordUseCaseImpl {
    #[instrument(name = "record_use_case/create", skip_all)]
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        title: &String,
        artist: &String,
        release_date: &String,
        cover_url: &String,
        discogs_url: Option<String>,
        spotify_url: Option<String>,
    ) -> Result<Record, AppError> {
        let record = repos
            .record
            .create(
                &mut *db_con,
                title,
                artist,
                release_date,
                cover_url,
                discogs_url,
                spotify_url,
            )
            .await?;
        Ok(record)
    }

    #[instrument(name = "record_use_case/find_all", skip_all)]
    async fn find_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<Record>, AppError> {
        let records = repos.record.find_all(&mut *db_con).await?;
        Ok(records)
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
}
