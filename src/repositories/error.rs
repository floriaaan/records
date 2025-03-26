use schemars::JsonSchema;
use thiserror::Error;

#[derive(Debug, Error, JsonSchema)]
pub enum DbRepoError {
    #[error("[DbRepoError::SerdeError] {0}")]
    #[serde(skip)]
    SerdeError(#[from] serde_json::Error),
    #[error("[DbRepoError::SqlxError] {0}")]
    #[serde(skip)]
    SqlxError(#[from] sqlx::Error),

    #[error("[DbRepoError::NotFound] Record not found")]
    NotFound,
    #[error("[DbRepoError::Conflict] Record already exists")]
    Conflict,
    #[error("[DbRepoError::Unauthorized] Unauthorized")]
    Unauthorized,

    #[cfg(test)]
    #[error("Dummy error for testing")]
    DummyTestError,
}
