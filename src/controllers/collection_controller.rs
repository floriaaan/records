use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::error::app_error::AppError;
use crate::models::collection_model::CollectionToken;
use crate::models::jwt_model::JwtClaim;
use crate::models::record_model::Record;
use crate::templating;
use crate::utils::Either;
use crate::utils::NetworkResponse;
use rocket::{get, post, delete, http::Status, response::content::RawHtml, serde::json::Json};
use serde::Serialize;
use tracing::instrument;

/// Generates a new collection token for an authenticated user
#[post("/tokens")]
#[instrument(name = "collection_controller/create_token", skip_all)]
async fn create_token(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<CollectionToken>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let token = app
        .use_cases
        .collection
        .create_token(&app.repos, &mut db, user_id)
        .await?;

    Ok(Json(token))
}

/// Lists all collection tokens for the authenticated user
#[get("/tokens")]
#[instrument(name = "collection_controller/list_tokens", skip_all)]
async fn list_tokens(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<CollectionToken>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let token = app
        .use_cases
        .collection
        .get_user_token(&app.repos, &mut db, user_id)
        .await?;

    Ok(Json(token))
}

/// Deletes a collection token
#[delete("/tokens/<token>")]
#[instrument(name = "collection_controller/delete_token", skip_all)]
async fn delete_token(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
    token: String,
) -> Result<Status, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    app.use_cases
        .collection
        .delete_token(&app.repos, &mut db, token, user_id)
        .await?;

    Ok(Status::NoContent)
}

// Define collection view template data structure
#[derive(Serialize)]
struct CollectionViewData {
    user_id: String,
    records: Vec<Record>,
    records_count: usize,
}

/// Gets a collection by its token in JSON format
#[get("/<token>?<owned>&<wanted>&<format>")]
#[instrument(name = "collection_controller/get_collection", skip_all)]
async fn get_collection(
    app: &AppState,
    mut db: ConnectionDb,
    token: String,
    format: Option<String>,
    owned: Option<bool>,
    wanted: Option<bool>,
) -> Result<Either<Json<Vec<Record>>, RawHtml<String>>, AppError> {
    let records = app
        .use_cases
        .collection
        .get_collection_by_token(&app.repos, &mut db, &token, owned, wanted)
        .await?;

    match format.as_deref() {
        Some("html") => {
            // Get user ID to display in the title
            let user_id = app
                .use_cases
                .collection
                .get_user_id_by_token(&app.repos, &mut db, &token)
                .await?;

            // Create data for the template
            let data = CollectionViewData {
                user_id: user_id.to_string(),
                records_count: records.len(),
                records: records.clone(),
            };

            // Render the template using the templating module
            let html = templating::render("collection_view", &data)?;
            Ok(Either::Right(RawHtml(html)))
        }
        _ => {
            // Default to JSON format
            Ok(Either::Left(Json(records)))
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_token, list_tokens, delete_token, get_collection]
}

#[cfg(test)]
mod tests {
    // Tests would go here
}