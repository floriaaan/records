// filepath: /Users/floriaaan/dev/records-rust/src/repositories/collection_token_repo.rs
use crate::models::collection_model::CollectionToken;
use crate::repositories::error::DbRepoError;
use crate::log_into;
use mockall::automock;
use sqlx::{query_as, PgConnection};
use tracing::instrument;

pub struct CollectionTokenRepoImpl {}

impl CollectionTokenRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait CollectionTokenRepo: Send + Sync {
    /// Create a new collection token for a user
    async fn create(
        &self,
        con: &mut PgConnection,
        user_id: i32,
    ) -> Result<CollectionToken, DbRepoError>;

    /// Find a collection token by its token string
    async fn find_by_token(
        &self,
        con: &mut PgConnection,
        token: &str,
    ) -> Result<Option<CollectionToken>, DbRepoError>;

    /// Find all tokens belonging to a user
    async fn find_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
    ) -> Result<CollectionToken, DbRepoError>;

    /// Delete a specific token
    async fn delete(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<(), DbRepoError>;

    /// Delete all tokens for a user
    async fn delete_all_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
    ) -> Result<(), DbRepoError>;
}

#[async_trait]
impl CollectionTokenRepo for CollectionTokenRepoImpl {
    #[instrument(name = "collection_token_repo/create", skip_all)]
    async fn create(
        &self,
        con: &mut PgConnection,
        user_id: i32,
    ) -> Result<CollectionToken, DbRepoError> {
        // Create a new token
        let collection_token = CollectionToken::new(user_id);

        // Save it to the database
        let saved_token = query_as!(
            CollectionToken,
            "INSERT INTO collection_tokens (token, user_id, created_at) VALUES ($1, $2, $3) RETURNING *",
            collection_token.token,
            collection_token.user_id,
            collection_token.created_at
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))?;

        Ok(saved_token)
    }

    #[instrument(name = "collection_token_repo/find_by_token", skip_all)]
    async fn find_by_token(
        &self,
        con: &mut PgConnection,
        token: &str,
    ) -> Result<Option<CollectionToken>, DbRepoError> {
        let collection_token = query_as!(
            CollectionToken,
            "SELECT * FROM collection_tokens WHERE token = $1",
            token
        )
        .fetch_optional(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))?;

        Ok(collection_token)
    }

    #[instrument(name = "collection_token_repo/find_by_user_id", skip_all)]
    async fn find_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
    ) -> Result<CollectionToken, DbRepoError> {
        let collection_token = query_as!(
            CollectionToken,
            "SELECT * FROM collection_tokens WHERE user_id = $1",
            user_id
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))?;

        Ok(collection_token)
    }

    #[instrument(name = "collection_token_repo/delete", skip_all)]
    async fn delete(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<(), DbRepoError> {
        sqlx::query!("DELETE FROM collection_tokens WHERE id = $1", id)
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;

        Ok(())
    }

    #[instrument(name = "collection_token_repo/delete_all_by_user_id", skip_all)]
    async fn delete_all_by_user_id(
        &self,
        con: &mut PgConnection,
        user_id: i32,
    ) -> Result<(), DbRepoError> {
        sqlx::query!("DELETE FROM collection_tokens WHERE user_id = $1", user_id)
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
    async fn test_create_token() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        
        let repo = CollectionTokenRepoImpl::new();
        let token = repo.create(&mut tx, 1).await;
        
        assert!(token.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_by_token() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        
        let repo = CollectionTokenRepoImpl::new();
        let token = repo.create(&mut tx, 1).await.unwrap();
        
        let found = repo.find_by_token(&mut tx, &token.token).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, token.id);
        
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_token() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        
        let repo = CollectionTokenRepoImpl::new();
        let token = repo.create(&mut tx, 1).await.unwrap();
        
        let result = repo.delete(&mut tx, token.id).await;
        assert!(result.is_ok());
        
        let not_found = repo.find_by_token(&mut tx, &token.token).await.unwrap();
        assert!(not_found.is_none());
        
        tx.rollback().await.unwrap();
    }
}