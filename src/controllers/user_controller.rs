use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::dto::user_dto::UserUpdateInput;
use crate::error::app_error::AppError;
use crate::models::jwt_model::JwtClaim;
use crate::models::user_model::User;
use crate::utils::NetworkResponse;
use rocket::serde::json::Json;
use tracing::instrument;

#[get("/")]
#[instrument(name = "user_controller/index", skip_all)]
async fn index(app: &AppState, mut db: ConnectionDb) -> Result<Json<Vec<User>>, AppError> {
    let users = app.use_cases.user.find_all(&app.repos, &mut db).await?;
    Ok(Json(users))
}

#[put("/", data = "<body>")]
#[instrument(name = "user_controller/update", skip_all)]
async fn update(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
    body: Json<UserUpdateInput>,
) -> Result<Json<User>, AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let input = body.into_inner();
    let user = app
        .use_cases
        .user
        .update(&app.repos, &mut db, user_id, &input.email, &input.username)
        .await?;
    Ok(Json(user))
}

#[delete("/")]
#[instrument(name = "user_controller/delete", skip_all)]
async fn delete(
    app: &AppState,
    mut db: ConnectionDb,
    jwt_claim: Result<JwtClaim, NetworkResponse>,
) -> Result<(), AppError> {
    let user_id = jwt_claim
        .map_err(|_| AppError::Unauthorized)
        .map(|key| key.sub)
        .map_err(|_| AppError::Unauthorized)?;

    app.use_cases
        .user
        .delete(&app.repos, &mut db, user_id)
        .await?;
    Ok(())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, update, delete]
}

#[cfg(test)]
mod tests {
    use crate::app_err;
    use crate::config::Config;
    use crate::db::Db;
    use crate::test::app::create_app_for_test;
    use crate::test::fixture::user::users_fixture;
    use crate::use_cases::user_use_case::MockUserUseCase;
    use rocket::fairing::AdHoc;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket_db_pools::Database;
    use std::sync::Arc;

    #[rocket::async_test]
    async fn test_index_success() {
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_find_all()
            .returning(|_, _| Ok(users_fixture(5)));

        let mut app_state = create_app_for_test();
        app_state.use_cases.user = Box::new(mock_user_use_case);

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
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_find_all()
            .returning(|_, _| app_err!(500, "error!"));

        let mut app_state = create_app_for_test();
        app_state.use_cases.user = Box::new(mock_user_use_case);

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
        assert_eq!(body_str, "error!");
    }
}
