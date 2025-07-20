use crate::application::errors::ApiError;
use crate::domains::categories::Category;
use crate::AppState;
use axum::extract::State;
use axum::{routing::get, Json, Router};
use crate::repositories::categories::Params;

pub fn create_routes() -> Router<AppState> {
    Router::new().route("/categories", get(get_categories))
}

pub async fn get_categories(State(state): State<AppState>) -> Result<Json<Vec<Category>>, ApiError> {
    let categories = state
        .category_repo
        .get_all(&state.db_pool, Params { ..Default::default() })
        .await?;
    Ok(Json(categories))
}
