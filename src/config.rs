use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub commands: Commands,
}

#[derive(Debug, Clone)]
pub enum CommandContext {
    Run,
    Test,
    Build,
    Bench,
    Script,
}

impl<'de> Deserialize<'de> for CommandContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommandContextVisitor;

        impl<'de> Visitor<'de> for CommandContextVisitor {
            type Value = CommandContext;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string for a command context")
            }

            fn visit_str<E>(self, value: &str) -> Result<CommandContext, E>
            where
                E: de::Error,
            {
                let context = match value.to_lowercase().as_str() {
                    "run" => CommandContext::Run,
                    "test" => CommandContext::Test,
                    "build" => CommandContext::Build,
                    "bench" => CommandContext::Bench,
                    _ => CommandContext::Script,
                };
                Ok(context)
            }
        }

        deserializer.deserialize_str(CommandContextVisitor)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commands {
    pub run: Option<CommandConfig>,
    pub test: Option<CommandConfig>,
    pub build: Option<CommandConfig>,
    pub bench: Option<CommandConfig>,
    pub script: Option<CommandConfig>,
}

#[derive(Debug, Clone)]
pub enum CommandType {
    Cargo,
    Shell,
    Script,
}

impl<'de> Deserialize<'de> for CommandType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "cargo" => Ok(CommandType::Cargo),
            "shell" => Ok(CommandType::Shell),
            _ => Ok(CommandType::Script), // Default to Script
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandConfig {
    pub default: String,
    pub configs: HashMap<String, CommandDetails>,
}

impl<'de> Deserialize<'de> for CommandConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommandConfigVisitor;

        impl<'de> Visitor<'de> for CommandConfigVisitor {
            type Value = CommandConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a command context")
            }

            fn visit_str<E>(self, value: &str) -> Result<CommandConfig, E>
            where
                E: de::Error,
            {
                Ok(CommandConfig::with_context(value))
            }
        }

        deserializer.deserialize_any(CommandConfigVisitor)
    }
}

impl CommandConfig {
    pub fn with_context(context: &str) -> Self {
        match context {
            "run" => Self {
                default: "default".into(),
                configs: {
                    let mut configs = HashMap::new();
                    configs.insert(
                        "default".to_string(),
                        CommandDetails {
                            command_type: Some(CommandType::Cargo),
                            command: Some("run".to_string()),
                            params: None,
                            env: None,
                            allow_multiple_instances: None,
                            working_directory: None,
                            pre_command: None,
                        },
                    );
                    configs
                },
            },
            "test" => Self {
                default: "default".into(),
                configs: {
                    let mut configs = HashMap::new();
                    configs.insert(
                        "default".to_string(),
                        CommandDetails {
                            command_type: Some(CommandType::Cargo),
                            command: Some("test".to_string()),
                            params: None,
                            env: None,
                            allow_multiple_instances: None,
                            working_directory: None,
                            pre_command: None,
                        },
                    );
                    configs
                },
            },
            "build" => Self {
                default: "default".into(),
                configs: {
                    let mut configs = HashMap::new();
                    configs.insert(
                        "default".to_string(),
                        CommandDetails {
                            command_type: Some(CommandType::Cargo),
                            command: Some("build".to_string()),
                            params: None,
                            env: None,
                            allow_multiple_instances: None,
                            working_directory: None,
                            pre_command: None,
                        },
                    );
                    configs
                },
            },
            _ => Self {
                default: "default".into(),
                configs: {
                    let mut configs = HashMap::new();
                    configs.insert(
                        "default".to_string(),
                        CommandDetails {
                            command_type: Some(CommandType::Shell),
                            command: None,
                            params: None,
                            env: None,
                            allow_multiple_instances: None,
                            working_directory: None,
                            pre_command: None,
                        },
                    );
                    configs
                },
            },
        }
    }
    pub fn update_config(&mut self, key: String, details: CommandDetails) {
        self.configs.insert(key, details);
    }
    pub fn remove_config(&mut self, key: &str) {
        self.configs.remove(key);
    }
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            default: "default".into(),
            configs: HashMap::new(), // An empty HashMap
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CommandDetails {
    #[serde(rename = "type")]
    pub command_type: Option<CommandType>,
    pub command: Option<String>,
    pub params: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub allow_multiple_instances: Option<bool>,
    pub working_directory: Option<String>,
    pub pre_command: Option<String>,
}

impl Default for Commands {
    fn default() -> Self {
        Commands {
            run: Some(CommandConfig::with_context("run")),
            test: Some(CommandConfig::with_context("test")),
            build: Some(CommandConfig::with_context("build")),
            bench: Some(CommandConfig::with_context("bench")),
            script: None,
        }
    }
}
