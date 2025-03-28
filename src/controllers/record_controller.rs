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


    // for record_input in inputs {
    //     record_input
    //         .validate()
    //         .map_err(|e| AppError::ValidationError { errors: e })?;

    //     let record = app
    //         .use_cases
    //         .record
    //         .create(&app.repos, &mut db, user_id, record_input)
    //         .await?;
    //     created_records.push(record);
    // }

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
    routes![index, add, get, random, search]
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
