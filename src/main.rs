pub mod application;
pub mod domains;
pub mod handlers;
pub mod repositories;

use crate::application::settings::SETTINGS;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = application::create().await;
    let port = SETTINGS.server.port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Server listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
