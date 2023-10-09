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
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on {}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
