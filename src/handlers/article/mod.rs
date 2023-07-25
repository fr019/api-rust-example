use crate::domains::article::{Article, CreateArticle};
use axum::http::StatusCode;
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use sqlx::PgPool;

pub fn create_routes() -> Router {
    Router::new()
        .route("/article", post(create_article))
        .route("/articles", get(get_articles))
        .route("/article/:id", delete(delete_article))
}

pub async fn create_article(
    Extension(pool): Extension<PgPool>,
    Json(new_article): Json<CreateArticle>,
) -> (StatusCode, Json<Article>) {
    let article = Article::create(&pool, new_article).await;
    (StatusCode::OK, Json(article))
}

pub async fn get_articles(Extension(pool): Extension<PgPool>) -> (StatusCode, Json<Vec<Article>>) {
    let articles = Article::get_all(&pool).await;
    (StatusCode::OK, Json(articles))
}

pub async fn delete_article(
    Extension(pool): Extension<PgPool>,
    Path(article_id): Path<i32>,
) -> (StatusCode, Json<Article>) {
    let article = Article::delete(&pool, article_id).await;
    (StatusCode::OK, Json(article))
}
