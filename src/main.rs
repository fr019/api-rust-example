pub mod application;
pub mod domains;
pub mod handlers;
pub mod repositories;
pub mod services;
pub mod validators;

use crate::application::logger;
use crate::application::settings::SETTINGS;
use crate::repositories::users::UsersRepo;
use crate::services::users::UsersService;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::repositories::categories::CategoriesRepo;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub users_service: Arc<UsersService>,
    pub users_repo: Arc<UsersRepo>,
    pub category_repo: Arc<CategoriesRepo>,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        let users_repo = Arc::new(UsersRepo);
        let users_service = Arc::new(UsersService::new(users_repo.clone()));
        let category_repo = Arc::new(CategoriesRepo);

        Self {
            db_pool,
            users_service,
            users_repo,
            category_repo,
        }
    }
}

#[tokio::main]
async fn main() {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    match run_mode.as_str() {
        "production" => {
            dotenvy::from_filename(".env.production").ok();
        }
        _ => {
            dotenvy::from_filename(".env.develop").ok();
        }
    };

    logger::setup();

    let app = application::create().await;
    let port = SETTINGS.server.port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
