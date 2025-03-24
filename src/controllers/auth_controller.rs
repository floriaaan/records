use crate::db::ConnectionDb;
use crate::dto::user_dto::UserInput;
use crate::error::app_error::AppError;
use crate::models::jwt_model::JwtClaim;
use crate::utils::NetworkResponse;
use crate::{app::AppState, models::jwt_model::Jwt};
use rocket::serde::json::Json;
use tracing::instrument;
use validator::Validate;

#[post("/login", data = "<body>")]
#[instrument(name = "auth_controller/log_in", skip_all)]
async fn log_in(
    app: &AppState,
    mut db: ConnectionDb,
    body: Json<UserInput>,
) -> Result<Json<Jwt>, AppError> {
    let body = body.into_inner();

    match body.validate() {
        Ok(_) => {}
        Err(e) => {
            let errors = e
                .field_errors()
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v))
                .collect::<Vec<String>>()
                .join(", ");
            return Err(AppError::ValidationError {
                message: errors,
            });
        }
    }

    let jwt = app
        .use_cases
        .auth
        .log_in(&app.repos, &mut db, &body.email, &body.password)
        .await?;

    Ok(Json(jwt))
}

#[post("/register", data = "<body>")]
#[instrument(name = "auth_controller/register", skip_all)]
async fn register(
    app: &AppState,
    mut db: ConnectionDb,
    body: Json<UserInput>,
) -> Result<Json<Jwt>, AppError> {
    let body = body.into_inner();

    match body.validate() {
        Ok(_) => {}
        Err(e) => {
            let errors = e
                .field_errors()
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v))
                .collect::<Vec<String>>()
                .join(", ");
            
            return Err(AppError::ValidationError {
                message: errors,
            });
        }
    }

    let jwt = app
        .use_cases
        .auth
        .register(&app.repos, &mut db, &body.email, &body.password)
        .await?;

    Ok(Json(jwt))
}

#[get("/me")]
#[instrument(name = "auth_controller/me", skip_all)]
async fn me(
    _app: &AppState,
    mut _db: ConnectionDb,
    key: Result<JwtClaim, NetworkResponse>,
) -> Result<Json<JwtClaim>, NetworkResponse> {
    let jwt_claim = key?;
    Ok(Json(jwt_claim))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![log_in, register, me]
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
