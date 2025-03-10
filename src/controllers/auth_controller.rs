use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::dto::user_dto::UserInput;
use crate::error::app_error::AppError;
use crate::models::jwt_model::Jwt;
use rocket::serde::json::Json;
use tracing::instrument;

#[post("/login", data = "<body>")]
#[instrument(name = "auth_controller/log_in", skip_all)]
async fn log_in(
    app: &AppState,
    mut db: ConnectionDb,
    body: Json<UserInput>,
) -> Result<Json<Jwt>, AppError> {
    let body = body.into_inner();
    let auth = app
        .use_cases
        .auth
        .log_in(&app.repos, &mut db, &body.email, &body.password)
        .await?;
    Ok(Json(auth))
}

#[post("/register", data = "<body>")]
#[instrument(name = "auth_controller/register", skip_all)]
async fn register(
    app: &AppState,
    mut db: ConnectionDb,
    body: Json<UserInput>,
) -> Result<Json<Jwt>, AppError> {
    let body = body.into_inner();
    app
        .use_cases
        .auth
        .register(&app.repos, &mut db, &body.email, &body.password)
        .await?;

    let auth = app
        .use_cases
        .auth
        .log_in(&app.repos, &mut db, &body.email, &body.password)
        .await?;
    Ok(Json(auth))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![log_in, register]
}

// #[cfg(test)]
// mod tests {
//     use crate::app_err;
//     use crate::config::Config;
//     use crate::db::Db;
//     use crate::test::app::create_app_for_test;
//     use crate::test::fixture::auth::auths_fixture;
//     use rocket::fairing::AdHoc;
//     use rocket::http::Status;
//     use rocket::local::asynchronous::Client;
//     use rocket_db_pools::Database;
//     use std::sync::Arc;

//     #[rocket::async_test]
//     async fn test_index_success() {
//         let mut mock_auth_use_case = MockUserUseCase::new();
//         mock_auth_use_case
//             .expect_find_all()
//             .returning(|_, _| Ok(auths_fixture(5)));

//         let mut app_state = create_app_for_test();
//         app_state.use_cases.auth = Box::new(mock_auth_use_case);

//         let rocket = rocket::build()
//             .manage(Arc::new(app_state))
//             .attach(Db::init())
//             .attach(AdHoc::config::<Config>())
//             .mount("/", routes![super::index]);
//         let client = Client::tracked(rocket)
//             .await
//             .expect("valid rocket instance");
//         let response = client.get("/").dispatch().await;

//         assert_eq!(response.status(), Status::Ok);
//     }

//     #[rocket::async_test]
//     async fn test_index_fail() {
//         let mut mock_auth_use_case = MockUserUseCase::new();
//         mock_auth_use_case
//             .expect_find_all()
//             .returning(|_, _| app_err!(500, "error!"));

//         let mut app_state = create_app_for_test();
//         app_state.use_cases.auth = Box::new(mock_auth_use_case);

//         let rocket = rocket::build()
//             .manage(Arc::new(app_state))
//             .attach(Db::init())
//             .attach(AdHoc::config::<Config>())
//             .mount("/", routes![super::index]);
//         let client = Client::tracked(rocket)
//             .await
//             .expect("valid rocket instance");
//         let response = client.get("/").dispatch().await;

//         assert_eq!(response.status(), Status::InternalServerError);
//         let body_str = response.into_string().await.expect("valid body string");
//         assert_eq!(body_str, "error!");
//     }
// }
