use super::error::optional_var;
use super::{ConfigError, ConfigNotice, HostSettings};

const APP_ENV: &str = "APP_ENV";
const DEFAULT_APP_ENV: &str = "development";

/// The whole app's configuration. Everything read from the environment lives
/// here, so nothing else in the crate touches `std::env`.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub app_env: String,
    pub host: HostSettings,
}

impl EngineConfig {
    /// Load `.env.<APP_ENV>` and build the config from the resulting
    /// environment. Real environment variables always win over the file.
    ///
    /// Returns the notices gathered along the way; logging is not up yet at
    /// this point, so `main` emits them once it is.
    pub fn from_env() -> Result<(Self, Vec<ConfigNotice>), ConfigError> {
        let (app_env, notices) = Self::load_env_file();

        let config = Self {
            app_env,
            host: HostSettings::from_env()?,
        };

        Ok((config, notices))
    }

    /// `APP_ENV` selects which env file to load, so it is read from the process
    /// environment before the file exists. Returns the environment name in
    /// effect afterwards.
    fn load_env_file() -> (String, Vec<ConfigNotice>) {
        let mut notices = Vec::new();

        let selected_env = optional_var(APP_ENV).unwrap_or_else(|| DEFAULT_APP_ENV.to_string());
        let env_file = format!(".env.{selected_env}");

        match dotenvy::from_filename(&env_file) {
            Ok(path) => notices.push(ConfigNotice::EnvFileLoaded { path }),
            Err(err) => notices.push(ConfigNotice::EnvFileMissing {
                file: env_file.clone(),
                reason: err.to_string(),
            }),
        }

        // APP_ENV may have just come from the file. dotenvy leaves an
        // already-set variable alone, so the shell still wins here.
        let app_env = optional_var(APP_ENV).unwrap_or_else(|| selected_env.clone());
        if app_env != selected_env {
            notices.push(ConfigNotice::AppEnvMismatch {
                file: env_file,
                declared: app_env.clone(),
                selected: selected_env,
            });
        }

        (app_env, notices)
    }
}
