use crate::log_into;
use crate::models::user_model::User;
use crate::repositories::error::DbRepoError;
use mockall::automock;
use sqlx::{query, query_as, PgConnection};
use tracing::instrument;

pub struct UserRepoImpl {}

impl UserRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(
        &self,
        con: &mut PgConnection,
        email: &String,
        username: &String,
        password: &String,
    ) -> Result<User, DbRepoError>;
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<User>, DbRepoError>;
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<User>, DbRepoError>;
    async fn find_by_email(
        &self,
        con: &mut PgConnection,
        email: &String,
    ) -> Result<Option<User>, DbRepoError>;
    async fn find_by_username(
        &self,
        con: &mut PgConnection,
        username: &String,
    ) -> Result<Option<User>, DbRepoError>;
    async fn update(
        &self,
        con: &mut PgConnection,
        id: i32,
        email: &String,
        username: &String,
    ) -> Result<User, DbRepoError>;
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError>;
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    #[instrument(name = "user_repo/create", skip_all)]
    async fn create(
        &self,
        con: &mut PgConnection,
        email: &String,
        username: &String,
        password: &String,
    ) -> Result<User, DbRepoError> {
        query_as!(
            User,
            "INSERT INTO users (email, username, password) VALUES ($1, $2, $3) RETURNING *",
            email,
            username,
            password
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError ))
    }

    #[instrument(name = "user_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<User>, DbRepoError> {
        let users = query_as!(User, "SELECT * FROM users")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(users)
    }

    #[instrument(name = "user_repo/find_by_id", skip_all, fields(id = %id))]
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<User>, DbRepoError> {
        query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/find_by_email", skip_all, fields(email = %email))]
    async fn find_by_email(
        &self,
        con: &mut PgConnection,
        email: &String,
    ) -> Result<Option<User>, DbRepoError> {
        query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/find_by_username", skip_all, fields(username = %username))]
    async fn find_by_username(
        &self,
        con: &mut PgConnection,
        username: &String,
    ) -> Result<Option<User>, DbRepoError> {
        query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/update", skip_all, fields(id = %id))]
    async fn update(
        &self,
        con: &mut PgConnection,
        id: i32,
        email: &String,
        username: &String,
    ) -> Result<User, DbRepoError> {
        query_as!(
            User,
            "UPDATE users SET email = $1, username = $2 WHERE id = $3 RETURNING *",
            email,
            username,
            id
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/delete", skip_all, fields(id = %id))]
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError> {
        query!("DELETE FROM users WHERE id = $1", id)
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::user_repo::{UserRepo, UserRepoImpl};
    use crate::test::db::create_db_con_for_test;
    use crate::test::repositories::prepare::user::create_user;
    use sqlx::Connection;

    #[tokio::test]
    async fn test_create_user() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let result = create_user(&mut tx).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_user_by_id() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let user = create_user(&mut tx).await.unwrap();
        let repo = UserRepoImpl::new();
        let result = repo.find_by_id(&mut tx, user.id).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_update_user() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let user = create_user(&mut tx).await.unwrap();
        let repo = UserRepoImpl::new();
        let new_email = "new_email@mail.com".to_string();
        let result = repo.update(&mut tx, user.id, &new_email, &user.username).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let user = create_user(&mut tx).await.unwrap();
        let repo = UserRepoImpl::new();
        let result = repo.delete(&mut tx, user.id).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }
}
