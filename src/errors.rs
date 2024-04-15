use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ConfigError {
    ConfigKey(String),
    PreCommand(String),
    Env(String),
    CommandKey(String),
    CommandType(String),
    // You can add more error variants as needed
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::ConfigKey(key) => {
                write!(f, "The config key '{}' does not exist.", key)
            }
            ConfigError::PreCommand(msg) => write!(f, "{}", msg),
            ConfigError::Env(msg) => write!(f, "{}", msg),
            ConfigError::CommandKey(msg) => write!(f, "{}", msg),
            ConfigError::CommandType(msg) => write!(f,"{}",msg),
        }
    }
}

impl std::error::Error for ConfigError {}
