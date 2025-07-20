pub mod db;
pub mod errors;
pub mod logger;
pub mod middlewares;
pub mod settings;

use crate::application::logger::trace_layer;
use crate::{handlers, AppState};
use axum::{http::header, Router};

use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
};

pub async fn create() -> Router {
    let pool = db::create_pool().await;
    let cors = CorsLayer::new().allow_origin(Any);
    let state = AppState::new(pool);

    Router::new()
        .merge(
            Router::new()
                .nest(
                    "/api/v1",
                    // All public v1 routes will be nested here.
                    Router::new().merge(handlers::users::create_routes()),
                )
                .with_state(state),
        )
        // High level logging of requests and responses
        .layer(trace_layer())
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        // CORS configuration. This should probably be more restrictive in
        // production.
        .layer(cors)
}
