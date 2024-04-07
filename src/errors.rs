use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ConfigError {
    ConfigKeyNotFound(String),
    InvalidPreCommand(String),
    // You can add more error variants as needed
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::ConfigKeyNotFound(key) => {
                write!(f, "The config key '{}' does not exist.", key)
            } // Handle other error variants as needed
            ConfigError::InvalidPreCommand(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}
