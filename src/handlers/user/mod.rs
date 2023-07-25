use crate::domains::user::{CreateUser, User};
use crate::domains::CollectionParams;
use axum::extract::Query;
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;

pub fn create_routes() -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/users", get(get_users))
        .route("/user/:id", delete(delete_user))
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(new_user): Json<CreateUser>,
) -> Json<User> {
    let user = User::create(&pool, new_user).await;
    Json(user)
}

pub async fn get_users(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<CollectionParams>,
) -> Json<Vec<User>> {
    let mut tx = pool.begin().await.unwrap();

    let records = User::get_all(&mut *tx, params.per_page, params.last_id, params.order).await;
    Json(records)
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Json<User> {
    let user = User::delete(&pool, user_id).await;
    Json(user)
}
