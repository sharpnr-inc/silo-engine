use std::sync::Arc;
use std::time::Instant;

use crate::config::EngineConfig;

/// Shared application state.
///
/// Dependencies are constructed once during startup and handed to the router;
/// handlers never build their own. Database pools, the storage backend and the
/// auth client join this struct as they land.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<EngineConfig>,
    started_at: Instant,
}

impl AppState {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config: Arc::new(config),
            started_at: Instant::now(),
        }
    }

    /// Seconds since the process finished starting up.
    pub fn uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}
