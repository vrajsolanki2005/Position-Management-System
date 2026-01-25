use axum::{routing::get, Router, Json};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio;

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "position-service",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Health check server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}