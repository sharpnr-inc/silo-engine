use std::path::PathBuf;

use tracing::{info, warn};

/// Something worth reporting from the configuration load.
///
/// Loading happens before the log subscriber exists -- the environment decides
/// how verbose logging should be -- so these are collected and returned rather
/// than logged in place. `main` emits them once logging is up.
#[derive(Debug, Clone)]
pub enum ConfigNotice {
    EnvFileLoaded {
        path: PathBuf,
    },
    EnvFileMissing {
        file: String,
        reason: String,
    },
    AppEnvMismatch {
        file: String,
        declared: String,
        selected: String,
    },
}

impl ConfigNotice {
    /// Log this notice at the severity its variant deserves.
    pub fn emit(&self) {
        match self {
            Self::EnvFileLoaded { path } => {
                info!("loaded config from {}", path.display());
            }
            Self::EnvFileMissing { file, reason } => {
                warn!("no config loaded from {file} ({reason}); using process environment");
            }
            Self::AppEnvMismatch {
                file,
                declared,
                selected,
            } => {
                warn!("{file} declares APP_ENV={declared}, but {selected} selected it");
            }
        }
    }
}
