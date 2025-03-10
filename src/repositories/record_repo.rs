use crate::log_into;
use crate::models::record_model::Record;
use crate::repositories::error::DbRepoError;
use mockall::automock;
use sqlx::{query, query_as, PgConnection};
use tracing::instrument;

pub struct RecordRepoImpl {}
impl RecordRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait RecordRepo: Send + Sync {
    async fn create(
        &self,
        con: &mut PgConnection,
        title: &String,
        artist: &String,
        release_date: &String,
        cover_url: &String,
        discogs_url: Option<String>,
        spotify_url: Option<String>,
    ) -> Result<Record, DbRepoError>;
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Record>, DbRepoError>;
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Record>, DbRepoError>;

    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError>;
}

#[async_trait]
impl RecordRepo for RecordRepoImpl {
    #[instrument(name = "record_repo/create", skip_all)]
    async fn create(
        &self,
        con: &mut PgConnection,
        title: &String,
        artist: &String,
        release_date: &String,
        cover_url: &String,
        discogs_url: Option<String>,
        spotify_url: Option<String>,
    ) -> Result<Record, DbRepoError> {
        let user_id = 1; // TODO: get user_id from token
        query_as!(
                    Record,
                    "INSERT INTO records (title, artist, release_date, cover_url, discogs_url, spotify_url, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
                    title,
                    artist,
                    chrono::NaiveDate::parse_from_str(release_date, "%Y-%m-%d").unwrap(),
                    cover_url,
                    discogs_url,
                    spotify_url,
                    user_id
                )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "record_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Record>, DbRepoError> {
        let records = query_as!(Record, "SELECT * FROM records")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(records)
    }

    #[instrument(name = "record_repo/find_by_id", skip_all, fields(id = %id))]
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Record>, DbRepoError> {
        query_as!(Record, "SELECT * FROM records WHERE id = $1", id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

    // #[instrument(name = "record_repo/update", skip_all, fields(id = %id))]
    // async fn update(
    //     &self,
    //     con: &mut PgConnection,
    //     id: i32,
    //     name: &String,
    // ) -> Result<Record, DbRepoError> {
    //     query_as!(
    //         Record,
    //         "UPDATE records SET name = $1 WHERE id = $2 RETURNING *",
    //         name,
    //         id
    //     )
    //     .fetch_one(&mut *con)
    //     .await
    //     .map_err(|e| log_into!(e, DbRepoError))
    // }

    #[instrument(name = "record_repo/delete", skip_all, fields(id = %id))]
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError> {
        query!("DELETE FROM records WHERE id = $1", id)
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::record_repo::{RecordRepo, RecordRepoImpl};
    use crate::test::db::create_db_con_for_test;
    use crate::test::repositories::prepare::record::create_record;
    use sqlx::Connection;

    #[tokio::test]
    async fn test_create_record() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let result = create_record(&mut tx).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_record_by_id() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let record = create_record(&mut tx).await.unwrap();
        let repo = RecordRepoImpl::new();
        let result = repo.find_by_id(&mut tx, record.id).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    

    #[tokio::test]
    async fn test_delete_record() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let record = create_record(&mut tx).await.unwrap();
        let repo = RecordRepoImpl::new();
        let result = repo.delete(&mut tx, record.id).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }
}
