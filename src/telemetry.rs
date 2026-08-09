use tracing_subscriber::EnvFilter;

/// Install the log subscriber. Nothing logged before this call is recorded, so
/// it runs as early in startup as the configuration allows.
///
/// `RUST_LOG` overrides the level when set; otherwise the default follows the
/// environment -- verbose in development, quiet in production.
pub fn init(app_env: &str) {
    let is_production = app_env == "production";

    let default_filter = if is_production {
        "silo_engine=info,tower_http=warn"
    } else {
        "silo_engine=debug,tower_http=debug,axum::rejection=trace"
    };

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(!is_production)
        .init();
}
