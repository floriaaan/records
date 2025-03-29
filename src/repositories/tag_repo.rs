use crate::log_into;
use crate::models::tag_model::Tag;
use crate::repositories::error::DbRepoError;
use mockall::automock;
use sqlx::{query, query_as, PgConnection};
use tracing::instrument;

pub struct TagRepoImpl {}

impl TagRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait TagRepo: Send + Sync {
    async fn create(
        &self,
        con: &mut PgConnection,
        name: &str,
    ) -> Result<Tag, DbRepoError>;
    
    async fn find_or_create(
        &self,
        con: &mut PgConnection,
        name: &str,
    ) -> Result<Tag, DbRepoError>;

    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Tag>, DbRepoError>;
    
    async fn find_by_slug(
        &self,
        con: &mut PgConnection,
        slug: &str,
    ) -> Result<Option<Tag>, DbRepoError>;
    
    async fn find_all(
        &self,
        con: &mut PgConnection,
    ) -> Result<Vec<Tag>, DbRepoError>;
    
    async fn find_all_by_record_id(
        &self,
        con: &mut PgConnection,
        record_id: i32,
    ) -> Result<Vec<Tag>, DbRepoError>;
    
    async fn associate_tags_with_record(
        &self,
        con: &mut PgConnection,
        record_id: i32,
        tag_ids: &[i32],
    ) -> Result<(), DbRepoError>;
}

#[async_trait]
impl TagRepo for TagRepoImpl {
    #[instrument(name = "tag_repo/create", skip_all)]
    async fn create(
        &self,
        con: &mut PgConnection,
        name: &str,
    ) -> Result<Tag, DbRepoError> {
        let tag = Tag::new(name.to_string());
        
        query_as!(
            Tag,
            "INSERT INTO tags (name, slug) VALUES ($1, $2) RETURNING *",
            tag.name,
            tag.slug
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }
    
    #[instrument(name = "tag_repo/find_or_create", skip_all)]
    async fn find_or_create(
        &self,
        con: &mut PgConnection,
        name: &str,
    ) -> Result<Tag, DbRepoError> {
        let slug = Tag::slugify(name);
        
        // Try to find existing tag by slug
        let existing_tag = self.find_by_slug(con, &slug).await?;
        
        if let Some(tag) = existing_tag {
            Ok(tag)
        } else {
            // Create new tag if not found
            self.create(con, name).await
        }
    }

    #[instrument(name = "tag_repo/find_by_id", skip_all, fields(id = %id))]
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Tag>, DbRepoError> {
        query_as!(Tag, "SELECT * FROM tags WHERE id = $1", id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }
    
    #[instrument(name = "tag_repo/find_by_slug", skip_all, fields(slug = %slug))]
    async fn find_by_slug(
        &self,
        con: &mut PgConnection,
        slug: &str,
    ) -> Result<Option<Tag>, DbRepoError> {
        query_as!(Tag, "SELECT * FROM tags WHERE slug = $1", slug)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }
    
    #[instrument(name = "tag_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Tag>, DbRepoError> {
        let tags = query_as!(Tag, "SELECT * FROM tags")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(tags)
    }
    
    #[instrument(name = "tag_repo/find_all_by_record_id", skip_all, fields(record_id = %record_id))]
    async fn find_all_by_record_id(
        &self,
        con: &mut PgConnection,
        record_id: i32,
    ) -> Result<Vec<Tag>, DbRepoError> {
        let tags = query_as!(
            Tag,
            "SELECT t.* FROM tags t 
             JOIN records_tags rt ON rt.tag_id = t.id
             WHERE rt.record_id = $1",
            record_id
        )
        .fetch_all(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(tags)
    }
    
    #[instrument(name = "tag_repo/associate_tags_with_record", skip_all, fields(record_id = %record_id))]
    async fn associate_tags_with_record(
        &self,
        con: &mut PgConnection,
        record_id: i32,
        tag_ids: &[i32],
    ) -> Result<(), DbRepoError> {
        // First, delete any existing associations (to handle updates correctly)
        query!("DELETE FROM records_tags WHERE record_id = $1", record_id)
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
            
        // If no tags to add, just return
        if tag_ids.is_empty() {
            return Ok(());
        }
            
        // Build a query to insert all associations in one go
        let mut query_string = String::from("INSERT INTO records_tags (record_id, tag_id) VALUES ");
        let mut values = Vec::new();
        
        for (i, tag_id) in tag_ids.iter().enumerate() {
            if i > 0 {
                query_string.push_str(", ");
            }
            query_string.push_str(&format!("($1, ${})", i + 2));
            values.push(*tag_id);
        }
        
        // Build the query with parameters
        let mut query_builder = sqlx::query(&query_string).bind(record_id);
        for tag_id in values {
            query_builder = query_builder.bind(tag_id);
        }
        
        query_builder
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
            
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::db::create_db_con_for_test;
    use sqlx::Connection;

    #[tokio::test]
    async fn test_create_tag() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        
        let repo = TagRepoImpl::new();
        let result = repo.create(&mut tx, "Test Tag").await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Test Tag");
        
        tx.rollback().await.unwrap();
    }
}