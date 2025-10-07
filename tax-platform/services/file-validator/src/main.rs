use tokio::net::TcpListener;
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("File validator service running on port 3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
