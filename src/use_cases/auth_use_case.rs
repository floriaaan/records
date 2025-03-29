use crate::error::app_error::AppError;
use crate::models::jwt_model::Jwt;
use crate::repositories::error::DbRepoError;
use crate::repositories::repositories::Repositories;
use crate::{db::DbCon, models::jwt_model::generate_jwt};
use bcrypt::{hash, verify};
use mockall::automock;
use tracing::instrument;

pub struct AuthUseCaseImpl {}
impl AuthUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait AuthUseCase: Send + Sync {
    async fn log_in(
        &self,
        repos: &Repositories,
        db_con: &mut DbCon,
        email: &String,
        password: &String,
    ) -> Result<Jwt, AppError>;
    async fn register(
        &self,
        repos: &Repositories,
        db_con: &mut DbCon,
        email: &String,
        password: &String,
    ) -> Result<Jwt, AppError>;
}

#[async_trait]
impl AuthUseCase for AuthUseCaseImpl {
    #[instrument(name = "auth_use_case/register", skip_all)]
    async fn register(
        &self,
        repos: &Repositories,
        db_con: &mut DbCon,
        email: &String,
        password: &String,
    ) -> Result<Jwt, AppError> {
        let hashed_password = hash(password, bcrypt::DEFAULT_COST).unwrap();

        let user = repos
            .user
            .create(&mut *db_con, email, &hashed_password)
            .await?;

        let jwt_claim = generate_jwt(user.id).await;

        let token = match jwt_claim {
            Ok(claim) => claim,
            Err(e) => return Err(AppError::new(500, &e.to_string())),
        };

        Ok(Jwt { token })
    }

    #[instrument(name = "auth_use_case/log_in", skip_all)]
    async fn log_in(
        &self,
        repos: &Repositories,
        db_con: &mut DbCon,
        email: &String,
        password: &String,
    ) -> Result<Jwt, AppError> {
        let user = repos.user.find_by_email(&mut *db_con, email).await?;
        if user.is_none() {
            return Err(AppError::CustomError {
                status_code: 401,
                message: "Invalid email or password".to_string(),
            });
        }

        let unwrapped_user = &user.unwrap();

        let is_valid = verify(password, &unwrapped_user.password).unwrap();
        if !is_valid {
            return Err(AppError::CustomError {
                status_code: 401,
                message: "Invalid email or password".to_string(),
            });
        }

        let jwt_claim = generate_jwt(unwrapped_user.id).await;

        let token = match jwt_claim {
            Ok(claim) => claim,
            Err(e) => return Err(AppError::new(500, &e.to_string())),
        };

        Ok(Jwt { token })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::app::create_repos_for_test;
    use crate::test::db::create_db_con_for_test;

    #[tokio::test]
    async fn test_register() {
        let repos = create_repos_for_test();
        let mut db_con = create_db_con_for_test().await.unwrap();
        let use_case = AuthUseCaseImpl::new();

        let email = "email".to_string();
        let password = "password".to_string();

        let jwt = use_case
            .register(&repos, &mut db_con, &email, &password)
            .await
            .unwrap();

        assert_eq!(jwt.token.len(), 183);
    }
}
