mod infrastructure;
mod data;
mod service;
mod app;
mod shared;

use crate::app::app::create_app;
use tracing::info;
use crate::infrastructure::logging::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize global logger first
    init_logger();

    info!("Starting pulp service");
    let app = create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.expect("Failed to bind TCP socket");

    info!("Listening on port 8000...");

    axum::serve(listener, app).await?;

    Ok(())
}