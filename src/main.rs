use axum::{Json, Router, routing::get};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let selected_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let env_file = format!(".env.{selected_env}");

    match dotenvy::from_filename(&env_file) {
        Ok(path) => println!("loaded config from {}", path.display()),
        Err(err) => println!("no config loaded from {env_file} ({err}); using process environment"),
    }

    // APP_ENV may have just come from the file. dotenvy leaves an already-set
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| selected_env.clone());
    if app_env != selected_env {
        println!("warning: {env_file} declares APP_ENV={app_env}, but {selected_env} selected it");
    }
    println!("starting silo-engine in {app_env} environment");

    let port: u16 = std::env::var("SILO_ENGINE_PORT")
        .expect("SILO_ENGINE_PORT must be set (see .env.example)")
        .parse()
        .expect("SILO_ENGINE_PORT must be a valid port number");

    let app = Router::new().route("/", get(hello_world));

    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}
