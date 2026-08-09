use std::env::VarError;
use std::fmt;

/// Everything that can go wrong while reading configuration out of the
/// environment. Each variant names the variable so startup failures say which
/// one to fix.
#[derive(Debug, Clone)]
pub enum ConfigError {
    Missing {
        name: &'static str,
    },
    Invalid {
        name: &'static str,
        value: String,
        reason: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { name } => {
                write!(f, "{name} must be set (see .env.example)")
            }
            Self::Invalid {
                name,
                value,
                reason,
            } => {
                write!(f, "{name}={value} is invalid: {reason}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Read a variable that the app cannot start without. A variable set to an
/// empty (or whitespace-only) value counts as missing -- that is nearly always
/// a half-filled env file rather than a deliberate choice.
pub(super) fn require_var(name: &'static str) -> Result<String, ConfigError> {
    match std::env::var(name) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        Ok(_) | Err(VarError::NotPresent) => Err(ConfigError::Missing { name }),
        Err(VarError::NotUnicode(_)) => Err(ConfigError::Invalid {
            name,
            value: "<non-utf8>".to_string(),
            reason: "value is not valid UTF-8".to_string(),
        }),
    }
}

/// Read an optional variable, treating empty values as unset.
pub(super) fn optional_var(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}
