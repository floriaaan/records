use crate::dto::record_dto::RecordInput;
use crate::models::record_model::Record;
use crate::repositories::error::DbRepoError;
use crate::repositories::record_repo::{RecordRepo, RecordRepoImpl};
use sqlx::postgres::PgConnection;

pub async fn create_record(db_con: &mut PgConnection) -> Result<Record, DbRepoError> {
    let record_repo = RecordRepoImpl::new();

    let title = "title".to_string();
    let artist = "artist".to_string();
    let release_date = "2021-01-01".to_string();
    let cover_url = "cover_url".to_string();
    let discogs_url = Some("discogs_url".to_string());
    let spotify_url = Some("spotify_url".to_string());
    let owned = true;
    let wanted = false;

    let user_id = 1;

    record_repo
        .create(
            &mut *db_con,
            user_id,
            RecordInput {
                title,
                artist,
                release_date,
                cover_url,
                discogs_url,
                spotify_url,
                owned: Some(owned),
                wanted: Some(wanted),
                tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            },
        )
        .await
}
