use crate::models::record_model::{Record, RecordDB};
use crate::models::tag_model::Tag;
use crate::repositories::error::DbRepoError;
use crate::{dto::record_dto::RecordInput, log_into};
use mockall::automock;
use sqlx::{query, query_as, Connection, PgConnection, Row};
use tracing::instrument;
use std::sync::OnceLock;
use crate::repositories::tag_repo::{TagRepo, TagRepoImpl};

// Global singleton instance of TagRepoImpl
static TAG_REPO: OnceLock<TagRepoImpl> = OnceLock::new();

// Helper function to get or initialize the TagRepo singleton
fn get_tag_repo() -> &'static dyn TagRepo {
    TAG_REPO.get_or_init(TagRepoImpl::new)
}

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
        record_input: RecordInput,
    ) -> Result<Record, DbRepoError>;
    async fn create_multiple(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        records_inputs: Vec<RecordInput>,
    ) -> Result<Vec<Record>, DbRepoError>;

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
        record_input: RecordInput,
    ) -> Result<Record, DbRepoError> {
        // Start a transaction to handle both record creation and tag association
        let mut tx = con.begin().await.map_err(|e| log_into!(e, DbRepoError))?;

        let record_db = query_as!(
            RecordDB,
            "INSERT INTO records (title, artist, release_date, cover_url, discogs_url, spotify_url, owned, wanted, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
            record_input.title,
            record_input.artist,
            chrono::NaiveDate::parse_from_str(&record_input.release_date, "%Y-%m-%d").unwrap(),
            record_input.cover_url,
            record_input.discogs_url,
            record_input.spotify_url,
            record_input.owned,
            record_input.wanted,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| log_into!(e, DbRepoError))?;

        // Convert RecordDB to Record
        let mut record = Record::from(record_db);
        let mut tags = Vec::<Tag>::new();

        if let Some(tag_names) = record_input.tags {
            if !tag_names.is_empty() {
                // Use the singleton tag repository
                let tag_repo = get_tag_repo();
                
                // Create or find each tag and collect their IDs
                let mut tag_ids = Vec::new();
                for tag_name in tag_names {
                    let tag = tag_repo.find_or_create(&mut tx, &tag_name).await?;
                    tags.push(tag.clone());
                    tag_ids.push(tag.id);
                }
                
                // Associate tags with the record
                if !tag_ids.is_empty() {
                    tag_repo.associate_tags_with_record(&mut tx, record.id, &tag_ids).await?;
                }
            }
        }

        // Commit the transaction
        tx.commit().await.map_err(|e| log_into!(e, DbRepoError))?;
        
        // Return the record with its associated tags
        Ok(record.with_tags(tags))
    }

    #[instrument(name = "record_repo/create_multiple", skip_all)]
    async fn create_multiple(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        records_inputs: Vec<RecordInput>,
    ) -> Result<Vec<Record>, DbRepoError> {
        if records_inputs.is_empty() {
            return Ok(Vec::new());
        }

        // Start a transaction to handle both records and tags
        let mut tx = con.begin().await.map_err(|e| log_into!(e, DbRepoError))?;
        
        // Build the SQL string with the proper number of placeholders
        let mut sql = String::from(
            "INSERT INTO records (title, artist, release_date, cover_url, discogs_url, spotify_url, owned, wanted, user_id) VALUES ",
        );
        let mut placeholders = Vec::with_capacity(records_inputs.len());
        for i in 0..records_inputs.len() {
            let base = i * 9;
            placeholders.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}, ${})",
                base + 1,
                base + 2,
                base + 3,
                base + 4,
                base + 5,
                base + 6,
                base + 7,
                base + 8,
                base + 9
            ));
        }
        sql.push_str(&placeholders.join(", "));
        sql.push_str(" RETURNING *");

        // Build the query and bind all parameters in order
        let mut query = sqlx::query(&sql);
        
        // Store tag names for each record for later association
        let mut record_tags: Vec<Option<Vec<String>>> = Vec::with_capacity(records_inputs.len());
        
        for record_input in records_inputs {
            let release_date =
                chrono::NaiveDate::parse_from_str(&record_input.release_date, "%Y-%m-%d").unwrap();
            query = query
                .bind(record_input.title)
                .bind(record_input.artist)
                .bind(release_date)
                .bind(record_input.cover_url)
                .bind(record_input.discogs_url)
                .bind(record_input.spotify_url)
                .bind(record_input.owned)
                .bind(record_input.wanted)
                .bind(user_id);
                
            // Store tags for later processing
            record_tags.push(record_input.tags);
        }

        // Execute the query to insert all records
        let rows = query
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;

        // Convert rows to RecordDB structs
        let records_db: Vec<RecordDB> = rows
            .iter()
            .map(|row| RecordDB {
                id: row.get("id"),
                title: row.get("title"),
                artist: row.get("artist"),
                release_date: row.get("release_date"),
                cover_url: row.get("cover_url"),
                discogs_url: row.get("discogs_url"),
                spotify_url: row.get("spotify_url"),
                owned: row.get("owned"),
                wanted: row.get("wanted"),
                user_id: row.get("user_id"),
            })
            .collect();
            
        // Convert RecordDB to Record
        let records: Vec<Record> = records_db.into_iter().map(Record::from).collect();
            
        // Process tags if they exist
        if record_tags.is_empty() {
            tx.commit().await.map_err(|e| log_into!(e, DbRepoError))?;
            return Ok(records);
        }

        // Use the singleton tag repository
        let tag_repo = get_tag_repo();

        let mut records_with_tags = Vec::with_capacity(records.len());
        
        // Associate each record with its tags
        for (record, tags_opt) in records.into_iter().zip(record_tags) {
            let mut tags = Vec::new();
            
            if let Some(tag_names) = tags_opt {
                if !tag_names.is_empty() {
                    // Create or find each tag and collect their IDs
                    let mut tag_ids = Vec::new();
                    for tag_name in tag_names {
                        let tag = tag_repo.find_or_create(&mut tx, &tag_name).await?;
                        tags.push(tag.clone());
                        tag_ids.push(tag.id);
                    }
                    
                    // Associate tags with the record
                    if !tag_ids.is_empty() {
                        tag_repo.associate_tags_with_record(&mut tx, record.id, &tag_ids).await?;
                    }
                }
            }
            
            // Add record with its tags to the result
            records_with_tags.push(record.with_tags(tags));
        }
        
        // Commit the transaction
        tx.commit().await.map_err(|e| log_into!(e, DbRepoError))?;

        Ok(records_with_tags)
    }

    #[instrument(name = "record_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Record>, DbRepoError> {
        // Use the singleton tag repository
        let tag_repo = get_tag_repo();
    
        // Get all records first
        let records_db = query_as!(RecordDB, "SELECT * FROM records")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
            
        // Convert RecordDB to Record
        let records: Vec<Record> = records_db.into_iter().map(Record::from).collect();
            
        // For each record, fetch its tags
        let mut records_with_tags = Vec::with_capacity(records.len());
        for record in records {
            let tags = tag_repo.find_all_by_record_id(con, record.id).await?;
            records_with_tags.push(record.with_tags(tags));
        }
        
        Ok(records_with_tags)
    }

    #[instrument(name = "record_repo/find_all_by_user_id", skip_all)]
    async fn find_all_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        owned: Option<bool>,
        wanted: Option<bool>,
    ) -> Result<Vec<Record>, DbRepoError> {
        // Use the singleton tag repository
        let tag_repo = get_tag_repo();
    
        // Build query with filter conditions
        let mut query_str = String::from("SELECT * FROM records WHERE user_id = $1");
        if let Some(owned) = owned {
            query_str.push_str(format!(" AND owned = {}", owned).as_str());
        }
        if let Some(wanted) = wanted {
            query_str.push_str(format!(" AND wanted = {}", wanted).as_str());
        }

        // Get records first
        let records_db = sqlx::query_as::<_, RecordDB>(&query_str)
            .bind(user_id)
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
            
        // Convert RecordDB to Record
        let records: Vec<Record> = records_db.into_iter().map(Record::from).collect();
            
        // For each record, fetch its tags
        let mut records_with_tags = Vec::with_capacity(records.len());
        for record in records {
            let tags = tag_repo.find_all_by_record_id(con, record.id).await?;
            records_with_tags.push(record.with_tags(tags));
        }
        
        Ok(records_with_tags)
    }

    #[instrument(name = "record_repo/find_by_id", skip_all, fields(id = %id))]
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Record>, DbRepoError> {
        // Use the singleton tag repository
        let tag_repo = get_tag_repo();
    
        // Try to find the record
        let record_db_opt = query_as!(RecordDB, "SELECT * FROM records WHERE id = $1", id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
            
        // If record found, get its tags
        if let Some(record_db) = record_db_opt {
            let record = Record::from(record_db);
            let tags = tag_repo.find_all_by_record_id(con, record.id).await?;
            Ok(Some(record.with_tags(tags)))
        } else {
            Ok(None)
        }
    }

    #[instrument(name = "record_repo/get_random_by_user_id", skip_all, fields(user_id = %user_id))]
    async fn get_random_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
        owned: Option<bool>,
        wanted: Option<bool>,
    ) -> Result<Option<Record>, DbRepoError> {
        // Use the singleton tag repository
        let tag_repo = get_tag_repo();
    
        // Build query with filter conditions
        let mut query_str = String::from("SELECT * FROM records WHERE user_id = $1");
        if let Some(owned) = owned {
            query_str.push_str(format!(" AND owned = {}", owned).as_str());
        }
        if let Some(wanted) = wanted {
            query_str.push_str(format!(" AND wanted = {}", wanted).as_str());
        }
        query_str.push_str(" ORDER BY RANDOM() LIMIT 1");

        // Try to find a random record
        let record_db_opt = sqlx::query_as::<_, RecordDB>(&query_str)
            .bind(user_id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
            
        // If record found, get its tags
        if let Some(record_db) = record_db_opt {
            let record = Record::from(record_db);
            let tags = tag_repo.find_all_by_record_id(con, record.id).await?;
            Ok(Some(record.with_tags(tags)))
        } else {
            Ok(None)
        }
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
