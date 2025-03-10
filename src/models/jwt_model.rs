use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub token: String,
    pub user_id: i32,

    pub issued_at: DateTime<chrono::Utc>,
    pub expires_at: DateTime<chrono::Utc>,
}
