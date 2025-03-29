// filepath: /Users/floriaaan/dev/records-rust/src/models/collection_model.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Collection Token model
/// Represents a token that allows access to a user's record collection
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CollectionToken {
    pub id: i32,
    pub token: String,  // UUID string
    pub user_id: i32,   // Associated user
    pub created_at: chrono::NaiveDateTime,
}

impl CollectionToken {
    /// Create a new collection token for a user
    pub fn new(user_id: i32) -> Self {
        let uuid = Uuid::new_v4();
        CollectionToken {
            id: 0, // Will be set by the database
            token: uuid.to_string(),
            user_id,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}