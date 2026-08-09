use std::net::{Ipv4Addr, SocketAddr};

use super::ConfigError;
use super::error::require_var;

/// Where this service listens.
#[derive(Debug, Clone)]
pub struct HostSettings {
    pub port: u16,
}

impl HostSettings {
    pub(super) fn from_env() -> Result<Self, ConfigError> {
        const PORT: &str = "SILO_ENGINE_PORT";

        let raw = require_var(PORT)?;
        let port = raw
            .trim()
            .parse::<u16>()
            .map_err(|err| ConfigError::Invalid {
                name: PORT,
                value: raw.clone(),
                reason: format!("not a valid port number ({err})"),
            })?;

        Ok(Self { port })
    }

    /// Address to bind the listener to.
    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::from((Ipv4Addr::UNSPECIFIED, self.port))
    }
}
