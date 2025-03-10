use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,

    // #[serde(skip)]
    pub created_at: chrono::NaiveDateTime,
}
