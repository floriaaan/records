use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::dto::record_dto::RecordInput;
use crate::error::app_error::AppError;
use crate::models::jwt_model::JwtClaim;
use crate::models::record_model::Record;
use crate::utils::NetworkResponse;
use rocket::serde::json::Json;
use tracing::instrument;
use validator::Validate;
use rocket::data::{Data, ToByteUnit};
use std::io::Cursor;
use csv::ReaderBuilder;

#[get("/?<owned>&<wanted>")]
#[instrument(name = "record_controller/index", skip_all)]
async fn index(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
    owned: Option<bool>,
    wanted: Option<bool>,
) -> Result<Json<Vec<Record>>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let records = app
        .use_cases
        .record
        .find_all_by_user_id(&app.repos, &mut db, user_id, owned, wanted)
        .await?;
    Ok(Json(records))
}

#[post("/", data = "<body>")]
#[instrument(name = "record_controller/add", skip_all)]
async fn add(
    app: &AppState,
    mut db: ConnectionDb,
    body: Json<Vec<RecordInput>>,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<Vec<Record>>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let inputs = body.into_inner();
    for record_input in &inputs {
        record_input
            .validate()
            .map_err(|e| AppError::ValidationError { errors: e })?;
    }

    let created_records = app
        .use_cases
        .record
        .create_multiple(&app.repos, &mut db, user_id, inputs)
        .await?;

    Ok(Json(created_records))
}

#[post("/import", data = "<data>")]
#[instrument(name = "record_controller/import", skip_all)]
async fn import(
    app: &AppState,
    mut db: ConnectionDb,
    data: Data<'_>,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<Vec<Record>>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    // Read data with a size limit of 5MB
    let bytes = match data.open(5.mebibytes()).into_bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return Err(AppError::new(500, &format!("Failed to read file: {}", e))),
    };
    
    if !bytes.is_complete() {
        return Err(AppError::new(413, "File too large (max 5MB)"));
    }

    // Convert bytes to string
    let string_data = match std::str::from_utf8(&bytes.value) {
        Ok(v) => v,
        Err(_) => return Err(AppError::new(400, "Invalid UTF-8 sequence")),
    };

    // Parse CSV
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(Cursor::new(string_data));

    // Transform CSV records into RecordInput objects
    let mut record_inputs = Vec::new();
    let mut import_errors = Vec::new();
    
    // Track the current row for error reporting
    let mut row_index = 0;
    
    for result in reader.records() {
        row_index += 1;
        
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                import_errors.push(format!("Row {}: Error reading CSV record: {}", row_index, e));
                continue;
            }
        };

        // Skip header row if present
        if record.get(0) == Some("Catalog#") {
            continue;
        }

        if record.len() < 7 {
            tracing::warn!("Row {}: Not enough fields (found {}, expected at least 7): {:?}", 
                row_index, record.len(), record);
            import_errors.push(format!("Row {}: Not enough fields (found {}, expected at least 7)", 
                row_index, record.len()));
            continue;
        }

        let artist = record.get(1).unwrap_or_default().trim();
        let title = record.get(2).unwrap_or_default().trim();
        let label = record.get(3).unwrap_or_default().trim();
        let release_year = record.get(6).unwrap_or_default().trim();
        let release_id = record.get(7).unwrap_or_default().trim();

        // Default coverUrl - could be updated with a real cover URL from an API call
        let cover_url = format!("https://via.placeholder.com/300x300?text={}",
            urlencoding::encode(&format!("{} - {}", artist, title)));

        // Format release date as YYYY-01-01 (using January 1st as default day/month)
        let release_date = if !release_year.is_empty() {
            format!("{}-01-01", release_year)
        } else {
            "2000-01-01".to_string() // Default date if not provided
        };

        // Create a discogs URL if release_id is available
        let discogs_url = if !release_id.is_empty() {
            Some(format!("https://www.discogs.com/release/{}", release_id))
        } else {
            None
        };

        let input = RecordInput {
            title: title.to_string(),
            artist: artist.to_string(),
            release_date,
            cover_url,
            discogs_url,
            spotify_url: None, // We don't have Spotify URL from Discogs CSV
            owned: Some(true), // Records in the collection are owned
            wanted: Some(false), // Not in wantlist since they're already owned
            tags: Some(vec![label.to_string()]), // Use label as a tag
        };

        // Validate the record input
        if let Err(e) = input.validate() {
            // Extract validation error messages for this record
            let validation_errors: Vec<String> = e.field_errors()
                .iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(|error| {
                        format!("Field '{}': {}", field, error.message.as_ref().unwrap_or(&"Invalid".into()))
                    }).collect::<Vec<String>>()
                })
                .collect();
                
            let error_msg = format!("Row {}: Validation errors: {}", 
                row_index, validation_errors.join(", "));
            import_errors.push(error_msg);
            continue;
        }

        record_inputs.push(input);
    }
    
    // If we have any errors, return them all together
    if !import_errors.is_empty() {
        return Err(AppError::new(400, &format!("Import failed with {} errors:\n{}", 
            import_errors.len(), 
            import_errors.join("\n"))));
    }
    
    // If no records were successfully parsed
    if record_inputs.is_empty() {
        return Err(AppError::new(400, "No valid records found in the CSV file"));
    }

    // Create the records in the database
    let created_records = app
        .use_cases
        .record
        .create_multiple(&app.repos, &mut db, user_id, record_inputs)
        .await?;

    Ok(Json(created_records))
}

#[get("/<id>")]
#[instrument(name = "record_controller/get", skip_all)]
async fn get(
    app: &AppState,
    mut db: ConnectionDb,
    id: i32,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<Option<Record>>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let record = match app
        .use_cases
        .record
        .find_by_id(&app.repos, &mut db, id)
        .await?
    {
        Some(record) => {
            if record.user_id != user_id {
                return Err(AppError::Unauthorized);
            }
            Some(record)
        }
        None => None,
    };

    Ok(Json(record))
}

#[get("/random?<owned>&<wanted>")]
#[instrument(name = "record_controller/random", skip_all)]
async fn random(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
    owned: Option<bool>,
    wanted: Option<bool>,
) -> Result<Json<Option<Record>>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let record = app
        .use_cases
        .record
        .get_random_by_user_id(&app.repos, &mut db, user_id, owned, wanted)
        .await?;
    Ok(Json(record))
}

#[get("/search?<query>")]
#[instrument(name = "record_controller/search", skip_all)]
async fn search(
    app: &AppState,
    query: String,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<Vec<Record>>, AppError> {
    let _user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let records = app.use_cases.record.search(&query).await?;
    Ok(Json(records))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, add, get, random, search, import]
}

#[cfg(test)]
mod tests {
    use crate::app_err;
    use crate::config::Config;
    use crate::db::Db;
    use crate::test::app::create_app_for_test;
    use crate::test::fixture::record::records_fixture;
    use crate::use_cases::record_use_case::MockRecordUseCase;
    use rocket::fairing::AdHoc;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket_db_pools::Database;
    use std::sync::Arc;

    #[rocket::async_test]
    async fn test_index_success() {
        let mut mock_record_use_case = MockRecordUseCase::new();
        mock_record_use_case
            .expect_find_all()
            .returning(|_, _| Ok(records_fixture(5)));

        let mut app_state = create_app_for_test();
        app_state.use_cases.record = Box::new(mock_record_use_case);

        let rocket = rocket::build()
            .manage(Arc::new(app_state))
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::index]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_index_fail() {
        let mut mock_record_use_case = MockRecordUseCase::new();
        mock_record_use_case
            .expect_find_all()
            .returning(|_, _| app_err!(500, "error"));

        let mut app_state = create_app_for_test();
        app_state.use_cases.record = Box::new(mock_record_use_case);

        let rocket = rocket::build()
            .manage(Arc::new(app_state))
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::index]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::InternalServerError);
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "error");
    }
}
