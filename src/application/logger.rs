use std::env;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup() {
    let level = env::var("LOGGER__LEVEL").unwrap_or_else(|_| "info".to_string());
    let sql_explain = env::var("LOGGER__SQL_EXPLAIN").unwrap_or_else(|_| "off".to_string());
    let env = format!("api_rust_example={level},tower_http={level},sqlx={sql_explain}");

    env::set_var("RUST_LOG", env);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("Logger configured successfully");
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}
