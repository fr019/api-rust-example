use crate::application::errors::ApiError;
use crate::domains::users::{CreateUser, LoginRequest, SearchQuery, User};
use crate::repositories::users::Params;
use crate::validators::{ValidJson, ValidQuery};
use crate::AppState;
use axum::extract::State;
use axum::{
    extract::Path, routing::{delete, get, post},
    Json,
    Router,
};
use hyper::StatusCode;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", delete(delete_user))
        .route("/auth/login", post(authenticate_user))
}

pub async fn create_user(
    State(state): State<AppState>,
    ValidJson(new_user): ValidJson<CreateUser>,
) -> Result<Json<User>, ApiError> {
    let user = state
        .users_service
        .create_user(&state.db_pool, new_user)
        .await?;
    Ok(Json(user))
}

pub async fn authenticate_user(
    State(state): State<AppState>,
    ValidJson(credentials): ValidJson<LoginRequest>,
) -> Result<Json<User>, ApiError> {
    let user = state
        .users_service
        .authenticate_user(&state.db_pool, credentials)
        .await?;
    Ok(Json(user))
}

pub async fn get_users(
    State(state): State<AppState>,
    ValidQuery(query): ValidQuery<SearchQuery>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = state
        .users_repo
        .get_all(
            &state.db_pool,
            Params {
                limit: query.limit,
                last_id: query.last_id,
                sort_by: query.sort_by,
                sort_order: query.sort_order,
                ..Default::default()
            },
        )
        .await?;
    Ok(Json(users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = state
        .users_service
        .get_user_by_id(&state.db_pool, user_id)
        .await?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = state.users_repo.delete(&state.db_pool, user_id).await?;
    Ok(Json(user))
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("the user for id `{0}` is not found")]
    NotFound(i32),
    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("password hashing error")]
    PasswordHashError,
    #[error("unknown user error")]
    Unknown,
}

impl UserError {
    pub fn prepare_api_error(self) -> (StatusCode, String) {
        match self {
            UserError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            UserError::EmailAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            UserError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            UserError::PasswordHashError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server error".to_string(),
            ),
            UserError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application;
    use axum::http::Request;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_user() {
        let app = application::create().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/user")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(json!({"name": "test_name", "email": "test@mail.ru" }).to_string())
                    .unwrap(),
            )
            .await
            .unwrap();

        println!("{:?}", &response);

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
    }
}
