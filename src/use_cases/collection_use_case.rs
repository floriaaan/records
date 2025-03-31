// filepath: /Users/floriaaan/dev/records-rust/src/use_cases/collection_use_case.rs
use crate::db::ConnectionDb;
use crate::error::app_error::AppError;
use crate::models::collection_model::CollectionToken;
use crate::models::record_model::Record;
use crate::repositories::repositories::Repositories;
use mockall::automock;
use tracing::instrument;

pub struct CollectionUseCaseImpl {}

impl CollectionUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait CollectionUseCase: Send + Sync {
    /// Create a new collection token for a user
    async fn create_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        user_id: i32
    ) -> Result<CollectionToken, AppError>;

    /// Get all tokens for a user
    async fn get_user_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        user_id: i32
    ) -> Result<CollectionToken, AppError>;

    /// Delete a token
    async fn delete_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        token: String, 
        user_id: i32
    ) -> Result<(), AppError>;

    /// Get a user's collection by token
    async fn get_collection_by_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        token: &str,
        owned: Option<bool>,
        wanted: Option<bool>
    ) -> Result<Vec<Record>, AppError>;

    /// Get user_id associated with a token
    async fn get_user_id_by_token(
        &self,
        repos: &Repositories,
        db: &mut ConnectionDb,
        token: &str
    ) -> Result<i32, AppError>;
}

#[async_trait]
impl CollectionUseCase for CollectionUseCaseImpl {
    #[instrument(name = "collection_use_case/create_token", skip_all)]
    async fn create_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        user_id: i32
    ) -> Result<CollectionToken, AppError> {
        let token = repos
            .collection_token
            .create(&mut **db, user_id)
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(token)
    }

    #[instrument(name = "collection_use_case/get_user_token", skip_all)]
    async fn get_user_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        user_id: i32
    ) -> Result<CollectionToken, AppError> {
        let token = repos
            .collection_token
            .find_by_user_id(&mut **db, user_id)
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(token)
    }

    #[instrument(name = "collection_use_case/delete_token", skip_all)]
    async fn delete_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        token: String, 
        user_id: i32
    ) -> Result<(), AppError> {
        // Get the token to verify ownership
        let token_opt = repos
            .collection_token
            .find_by_token(&mut **db, &token)
            .await
            .map_err(|e| AppError::from(e))?;
            
        let token = token_opt.ok_or(AppError::NotFound)?;
        
        // Verify token ownership
        if token.user_id != user_id {
            return Err(AppError::Unauthorized);
        }
        
        // Delete the token
        repos
            .collection_token
            .delete(&mut **db, token.id)
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(())
    }

    #[instrument(name = "collection_use_case/get_collection_by_token", skip_all)]
    async fn get_collection_by_token(
        &self, 
        repos: &Repositories, 
        db: &mut ConnectionDb, 
        token: &str,
        owned: Option<bool>,
        wanted: Option<bool>
    ) -> Result<Vec<Record>, AppError> {
        // Find the token to get the user_id
        let token_opt = repos
            .collection_token
            .find_by_token(&mut **db, token)
            .await
            .map_err(|e| AppError::from(e))?;
            
        let user_token = token_opt.ok_or(AppError::NotFound)?;
        
        // Get the user's collection
        let records = repos
            .record
            .find_all_by_user_id(&mut **db, user_token.user_id, owned, wanted)
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(records)
    }

    #[instrument(name = "collection_use_case/get_user_id_by_token", skip_all)]
    async fn get_user_id_by_token(
        &self,
        repos: &Repositories,
        db: &mut ConnectionDb,
        token: &str
    ) -> Result<i32, AppError> {
        // Find the token to get the user_id
        let token_opt = repos
            .collection_token
            .find_by_token(&mut **db, token)
            .await
            .map_err(|e| AppError::from(e))?;
            
        let user_token = token_opt.ok_or(AppError::NotFound)?;
        
        Ok(user_token.user_id)
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_create_token() {
        // This test would require mocking the repository
        // and is left as an exercise
    }
}