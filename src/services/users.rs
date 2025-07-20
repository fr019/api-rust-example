use crate::domains::users::{CreateUser, LoginRequest, User};
use crate::handlers::users::UserError;
use crate::repositories::users::{Params, UsersRepo};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher};
use password_hash::PasswordVerifier;
use sqlx::PgExecutor;
use std::sync::Arc;

pub struct UsersService {
    users_repo: Arc<UsersRepo>,
}

impl UsersService {
    pub fn new(users_repo: Arc<UsersRepo>) -> Self {
        Self { users_repo }
    }

    pub async fn create_user(
        &self,
        executor: impl PgExecutor<'_>,
        mut new_user: CreateUser,
    ) -> Result<User, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .map_err(|_| UserError::PasswordHashError)?
            .to_string();

        new_user.email = new_user.email.to_lowercase().trim().to_string();
        new_user.password = password_hash;
        self.users_repo.create(executor, new_user).await
    }

    pub async fn authenticate_user(
        &self,
        executor: impl PgExecutor<'_>,
        credentials: LoginRequest,
    ) -> Result<User, UserError> {
        let normalized_email = credentials.email.to_lowercase().trim().to_string();

        let user = self
            .users_repo
            .get_all(
                executor,
                Params {
                    email: Some(normalized_email),
                    limit: Some(1),
                    ..Default::default()
                },
            )
            .await?
            .pop()
            .ok_or(UserError::InvalidCredentials)?;

        // Проверяем пароль
        let parsed_hash = PasswordHash::new(&user.password).map_err(|e| {
            tracing::error!("Corrupted password hash in DB: {}", e);
            UserError::InvalidCredentials
        })?;

        let argon2 = Argon2::default();
        argon2
            .verify_password(credentials.password.as_bytes(), &parsed_hash)
            .map_err(|_| UserError::InvalidCredentials)?;

        Ok(user)
    }

    pub async fn get_user_by_id(
        &self,
        executor: impl PgExecutor<'_>,
        user_id: i32,
    ) -> Result<User, UserError> {
        let mut users = self
            .users_repo
            .get_all(
                executor,
                Params {
                    ids: Some(vec![user_id]),
                    limit: Some(1),
                    ..Default::default()
                },
            )
            .await?;

        users.pop().ok_or(UserError::NotFound(user_id))
    }
}
