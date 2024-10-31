use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ConfigError {
    MissingConfigFields(String),
    ConfigKeyNotFound(String),
    InvalidSubCommand(String),
    InvalidEnvFormat,
    // You can add more error variants as needed
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingConfigFields(str) => {
                write!(f, "Missing Config Fields: {}", str)
            }
            ConfigError::ConfigKeyNotFound(key) => {
                write!(f, "The config key '{}' does not exist.", key)
            }
            ConfigError::InvalidSubCommand(msg) => write!(f, "{}", msg),
            ConfigError::InvalidEnvFormat => write!(f, "ENV define is not ALL_CAPS"),
        }
    }
}

impl std::error::Error for ConfigError {}
