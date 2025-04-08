use crate::application::settings::SETTINGS;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn create_pool() -> PgPool {
    let database_url = SETTINGS.database.url.as_str();
    tracing::info!("Connecting to database at: {}", database_url);

    env::set_var("DATABASE_URL", database_url);

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(SETTINGS.database.max_connections)
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("Failed to create Postgres connection pool! URL: {}", url));

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to make migrations.");

    pool
}
