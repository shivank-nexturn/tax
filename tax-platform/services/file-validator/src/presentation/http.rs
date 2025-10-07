use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};

pub fn create_router() -> Router<()> {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "file-validator"
    }))
}

async fn get_metrics() -> Json<Value> {
    Json(json!({
        "files_processed": 0,
        "files_validated": 0,
        "files_failed": 0
    }))
}
