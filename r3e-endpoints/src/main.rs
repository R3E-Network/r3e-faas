// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::net::SocketAddr;

use r3e_endpoints::{config::Config, create_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = Config::from_env()?;

    // Create the application
    let app = create_app(config.clone()).await?;

    // Get the address to bind to
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Start the server
    log::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
