use axum::{Json, Router, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

const DEFAULT_PORT: u16 = 9254;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let port: u16 = std::env::var("SILO_ENGINE_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}
