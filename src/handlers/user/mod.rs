use axum::extract::Query;
use axum::response::{IntoResponse};
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use hyper::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use crate::application::errors::ApiError;
use crate::domains::user::{CreateUser, User};
use crate::domains::CollectionParams;

pub fn create_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", delete(delete_user))
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, ApiError> {
    let user = User::create(&pool, new_user).await?;
    Ok(Json(user))
}

pub async fn get_users(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<CollectionParams>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = User::get_all(&pool, params.per_page, params.last_id, params.order).await?;
    Ok(Json(users))
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = User::get(&pool, user_id).await?;

    Ok(Json(user))
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = User::delete(&pool, user_id).await?;
    Ok(Json(user))
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("the user for id `{0}` is not found")]
    NotFound(i32),
    #[error("unknown user error")]
    Unknown,
}

impl UserError {
    pub fn prepare_api_error(self) -> (StatusCode, String) {
        match self {
            UserError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
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
