use tokio::net::TcpListener;
use tracing::{error, info};

mod api;
mod config;
mod state;
mod telemetry;

use config::EngineConfig;
use state::AppState;

#[tokio::main]
async fn main() {
    // Configuration comes first: it decides how verbose logging should be.
    // Its notices are held back until the subscriber exists, just below.
    let (config, notices) = match EngineConfig::from_env() {
        Ok(loaded) => loaded,
        Err(err) => {
            eprintln!("configuration error: {err}");
            std::process::exit(1);
        }
    };

    telemetry::init(&config.app_env);

    for notice in &notices {
        notice.emit();
    }

    info!(environment = %config.app_env, "starting silo-engine");

    let addr = config.host.bind_addr();
    let app = api::build(AppState::new(config));

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            error!("could not bind {addr}: {err}");
            std::process::exit(1);
        }
    };

    let local_addr = listener.local_addr().unwrap_or(addr);
    info!("listening on {local_addr}");
    info!("docs: http://{local_addr}{}", api::router::SWAGGER_UI_PATH);

    if let Err(err) = axum::serve(listener, app).await {
        error!("server error: {err}");
        std::process::exit(1);
    }
}
