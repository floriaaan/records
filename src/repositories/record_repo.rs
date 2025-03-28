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
        user_id: i32,
        title: &String,
        artist: &String,
        release_date: &String,
        cover_url: &String,
        discogs_url: Option<String>,
        spotify_url: Option<String>,
        owned: bool,
        wanted: bool,
    ) -> Result<Record, DbRepoError>;
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Record>, DbRepoError>;
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Record>, DbRepoError>;
    async fn find_all_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        owned: Option<bool>,
        wanted: Option<bool>,
    ) -> Result<Vec<Record>, DbRepoError>;
    // async fn update(&self, con: &mut PgConnection, id: i32, name: &String) -> Result<Record, DbRepoError>;
    async fn get_random_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        owned: Option<bool>,
        wanted: Option<bool>,
    ) -> Result<Option<Record>, DbRepoError>;

    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError>;
}

#[async_trait]
impl RecordRepo for RecordRepoImpl {
    #[instrument(name = "record_repo/create", skip_all)]
    async fn create(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        title: &String,
        artist: &String,
        release_date: &String,
        cover_url: &String,
        discogs_url: Option<String>,
        spotify_url: Option<String>,
        owned: bool,
        wanted: bool,
    ) -> Result<Record, DbRepoError> {
        query_as!(
                    Record,
                    "INSERT INTO records (title, artist, release_date, cover_url, discogs_url, spotify_url, owned, wanted, user_id) VALUES ($1::TEXT, $2::TEXT, $3::DATE, $4::TEXT, $5::TEXT, $6::TEXT, $7::BOOLEAN, $8::BOOLEAN, $9::INTEGER) RETURNING *",
                    title,
                    artist,
                    chrono::NaiveDate::parse_from_str(release_date, "%Y-%m-%d").unwrap(),
                    cover_url,
                    discogs_url,
                    spotify_url,
                    owned,
                    wanted,
                    user_id
                )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError)).map(
            |record| {
                Record {
                    id: record.id,
                    title: record.title,
                    artist: record.artist,
                    release_date: record.release_date,
                    cover_url: record.cover_url,
                    discogs_url: record.discogs_url,
                    spotify_url: record.spotify_url,
                    owned: record.owned,
                    wanted: record.wanted,
                    user_id: record.user_id,
                }
            },
        )
    }

    #[instrument(name = "record_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Record>, DbRepoError> {
        let records = query_as!(Record, "SELECT * FROM records")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(records)
    }

    #[instrument(name = "record_repo/find_all_by_user_id", skip_all)]
    async fn find_all_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        owned: Option<bool>,
        wanted: Option<bool>,
    ) -> Result<Vec<Record>, DbRepoError> {
        let mut query = String::from("SELECT * FROM records WHERE user_id = $1");
        if let Some(owned) = owned {
            query.push_str(format!(" AND owned = {}", owned).as_str());
        }
        if let Some(wanted) = wanted {
            query.push_str(format!(" AND wanted = {}", wanted).as_str());
        }

        let records = query_as::<_, Record>(&query)
            .bind(user_id)
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

    #[instrument(name = "record_repo/get_random_by_user_id", skip_all, fields(user_id = %user_id))]
    async fn get_random_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        owned: Option<bool>,
        wanted: Option<bool>,
    ) -> Result<Option<Record>, DbRepoError> {
        let mut query = String::from("SELECT * FROM records WHERE user_id = $1");
        if let Some(owned) = owned {
            query.push_str(format!(" AND owned = {}", owned).as_str());
        }
        if let Some(wanted) = wanted {
            query.push_str(format!(" AND wanted = {}", wanted).as_str());
        }
        query.push_str(" ORDER BY RANDOM() LIMIT 1");

        query_as::<_, Record>(&query)
            .bind(user_id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

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
