use axum::{Json, Router, routing::get};
use serde_json::json;
use tokio::net::TcpListener;
mod config;

use config::EngineConfig;

#[tokio::main]
async fn main() {
    let config = match EngineConfig::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("configuration error: {err}");
            std::process::exit(1);
        }
    };

    println!("starting silo-engine in {} environment", config.app_env);

    let app = Router::new().route("/", get(hello_world));

    let listener = TcpListener::bind(config.host.bind_addr()).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}
