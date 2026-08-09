use super::error::optional_var;
use super::{ConfigError, HostSettings};

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
    pub fn from_env() -> Result<Self, ConfigError> {
        let app_env = Self::load_env_file();

        Ok(Self {
            app_env,
            host: HostSettings::from_env()?,
        })
    }

    /// `APP_ENV` selects which env file to load, so it is read from the process
    /// environment before the file exists. Returns the environment name in
    /// effect afterwards.
    fn load_env_file() -> String {
        let selected_env = optional_var(APP_ENV).unwrap_or_else(|| DEFAULT_APP_ENV.to_string());
        let env_file = format!(".env.{selected_env}");

        match dotenvy::from_filename(&env_file) {
            Ok(path) => println!("loaded config from {}", path.display()),
            Err(err) => {
                println!("no config loaded from {env_file} ({err}); using process environment")
            }
        }

        // APP_ENV may have just come from the file. dotenvy leaves an
        // already-set variable alone, so the shell still wins here.
        let app_env = optional_var(APP_ENV).unwrap_or_else(|| selected_env.clone());
        if app_env != selected_env {
            println!(
                "warning: {env_file} declares {APP_ENV}={app_env}, but {selected_env} selected it"
            );
        }

        app_env
    }
}
