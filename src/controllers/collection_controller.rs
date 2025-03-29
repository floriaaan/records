use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::error::app_error::AppError;
use crate::models::collection_model::CollectionToken;
use crate::models::jwt_model::JwtClaim;
use crate::models::record_model::Record;
use crate::utils::NetworkResponse;
use rocket::{get, post, delete, http::ContentType, http::Status, response::content::RawHtml, serde::json::Json};
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

            // Generate HTML response
            let html = generate_html_collection(user_id, &records);
            Ok(Either::Right(RawHtml(html)))
        }
        _ => {
            // Default to JSON format
            Ok(Either::Left(Json(records)))
        }
    }
}

// Define a custom Either type to handle different response types
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// Implement Responder for Either
impl<'r, L: rocket::response::Responder<'r, 'static>, R: rocket::response::Responder<'r, 'static>> rocket::response::Responder<'r, 'static> for Either<L, R> {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Either::Left(left) => left.respond_to(req),
            Either::Right(right) => right.respond_to(req),
        }
    }
}

/// Generate HTML for collection view
fn generate_html_collection(user_id: i32, records: &[Record]) -> String {
    let mut html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Record Collection - User {}</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f7f7f7;
        }}
        h1, h2 {{
            color: #222;
        }}
        .collection-info {{
            margin-bottom: 30px;
            padding: 15px;
            background-color: #fff;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        .records-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 20px;
        }}
        .record-card {{
            background-color: #fff;
            border-radius: 5px;
            overflow: hidden;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            transition: transform 0.3s ease;
        }}
        .record-card:hover {{
            transform: translateY(-5px);
        }}
        .record-cover {{
            width: 100%;
            height: 200px;
            object-fit: cover;
        }}
        .record-info {{
            padding: 15px;
        }}
        .record-title {{
            font-size: 16px;
            font-weight: bold;
            margin: 0 0 5px 0;
        }}
        .record-artist {{
            font-size: 14px;
            color: #666;
            margin: 0 0 10px 0;
        }}
        .record-date {{
            font-size: 12px;
            color: #999;
        }}
        .badge {{
            display: inline-block;
            padding: 3px 8px;
            border-radius: 3px;
            font-size: 12px;
            margin-right: 5px;
            color: white;
        }}
        .owned {{
            background-color: #4CAF50;
        }}
        .wanted {{
            background-color: #2196F3;
        }}
        .tag {{
            display: inline-block;
            background-color: #e9e9e9;
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 11px;
            margin-right: 4px;
            margin-bottom: 4px;
            color: #555;
        }}
        .tags-container {{
            margin-top: 8px;
        }}
    </style>
</head>
<body>
    <div class="collection-info">
        <h1>Record Collection</h1>
        <p>User ID: {}</p>
        <p>Total Records: {}</p>
    </div>

    <div class="records-grid">
"#,
        user_id, user_id, records.len()
    );

    for record in records {
        let mut badges = String::new();
        if record.owned {
            badges.push_str(r#"<span class="badge owned">Owned</span>"#);
        }
        if record.wanted {
            badges.push_str(r#"<span class="badge wanted">Wanted</span>"#);
        }

        let mut tags_html = String::new();
        if let Some(tags) = &record.tags {
            if !tags.is_empty() {
                tags_html.push_str(r#"<div class="tags-container">"#);
                for tag in tags {
                    tags_html.push_str(&format!(r#"<span class="tag">{}</span>"#, tag.name));
                }
                tags_html.push_str("</div>");
            }
        }

        html.push_str(&format!(
            r#"
        <div class="record-card">
            <img src="{}" alt="{}" class="record-cover">
            <div class="record-info">
                <h3 class="record-title">{}</h3>
                <p class="record-artist">{}</p>
                <p class="record-date">{}</p>
                <div>{}</div>
                {}
            </div>
        </div>
    "#,
            record.cover_url,
            record.title,
            record.title,
            record.artist,
            record.release_date,
            badges,
            tags_html
        ));
    }

    html.push_str(
        r#"
    </div>
</body>
</html>
"#,
    );

    html
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_token, list_tokens, delete_token, get_collection]
}

#[cfg(test)]
mod tests {
    // Tests would go here
}