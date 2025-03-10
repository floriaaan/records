use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::dto::record_dto::RecordInput;
use crate::error::app_error::AppError;
use crate::models::record_model::Record;
use rocket::serde::json::Json;
use tracing::instrument;

#[get("/")]
#[instrument(name = "record_controller/index", skip_all)]
async fn index(app: &AppState, mut db: ConnectionDb) -> Result<Json<Vec<Record>>, AppError> {
    let records = app.use_cases.record.find_all(&app.repos, &mut db).await?;
    Ok(Json(records))
}

#[post("/", data = "<body>")]
#[instrument(name = "record_controller/add", skip_all)]
async fn add(
    app: &AppState,
    mut db: ConnectionDb,
    body: Json<RecordInput>,
) -> Result<Json<Record>, AppError> {
    let body = body.into_inner();
    let title = body.title;
    let artist = body.artist;
    let release_date = body.release_date;
    let cover_url = body.cover_url;
    let discogs_url = body.discogs_url;
    let spotify_url = body.spotify_url;

    let record = app
        .use_cases
        .record
        .create(
            &app.repos,
            &mut db,
            &title,
            &artist,
            &release_date,
            &cover_url,
            discogs_url,
            spotify_url,
        )
        .await?;
    Ok(Json(record))
}

#[get("/<id>")]
#[instrument(name = "record_controller/get", skip_all)]
async fn get(
    app: &AppState,
    mut db: ConnectionDb,
    id: i32,
) -> Result<Json<Option<Record>>, AppError> {
    let record = app
        .use_cases
        .record
        .find_by_id(&app.repos, &mut db, id)
        .await?;
    Ok(Json(record))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, add, get]
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
