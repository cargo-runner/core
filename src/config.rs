use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use toml;

use crate::global::{
    CONFIGURATION_FILE_CONTENT, DEFAULT_BENCH_CONFIG, DEFAULT_BUILD_CONFIG, DEFAULT_CONFIG_PATH,
    DEFAULT_RUN_CONFIG, DEFAULT_SCRIPT_CONFIG, DEFAULT_TEST_CONFIG,
};
use crate::helper::{read_file, write_to_config_file};

#[derive(Debug, Serialize, Clone, Default)]
pub enum CommandContext {
    Run,
    Test,
    Build,
    Bench,
    #[default]
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

#[derive(Debug, Serialize, Clone, Default)]
pub enum CommandType {
    Cargo,
    #[default]
    Shell,
}

// Implement Deserialize manually to include default behavior
impl<'de> Deserialize<'de> for CommandType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Use a custom visitor or another deserialization approach
        // that defaults to Shell for unrecognized values
        let s: Option<String> = Option::deserialize(deserializer)?;
        Ok(match s.as_deref() {
            Some("cargo") => CommandType::Cargo,
            _ => CommandType::Shell,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub commands: Commands,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Config, Box<dyn Error>> {
        if let Some(file_path) = path {
            read_file(file_path.as_path())?;
        } else {
            read_file(DEFAULT_CONFIG_PATH.get().unwrap())?;
        }

        let file_content = CONFIGURATION_FILE_CONTENT.lock().unwrap();

        let config: Config = toml::from_str(&file_content).unwrap_or(Config::default());
        Ok(config)
    }

    pub fn save(&self, path: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
        // Determine the file path to use: provided path or default
        let file_path = path.unwrap_or_else(|| {
            DEFAULT_CONFIG_PATH
                .get()
                .expect("DEFAULT_CONFIG_PATH not set")
                .clone()
        });

        // We need Config Struct and all Other Fields (struct or enum) to be impl Serialize
        let toml_string = toml::to_string_pretty(&self)?;

        // Write the serialized string to the file line by line
        write_to_config_file(&file_path, &toml_string)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commands {
    pub run: Option<CommandConfig>,
    pub test: Option<CommandConfig>,
    pub build: Option<CommandConfig>,
    pub bench: Option<CommandConfig>,
    pub script: Option<CommandConfig>,
}

impl Commands {
    pub fn get_command_config<'a>(
        &'a self,
        command_context: CommandContext,
        config_name: &str,
    ) -> Option<&'a CommandDetails> {
        let command_config = match command_context {
            CommandContext::Run => self.run.as_ref().or_else(|| DEFAULT_RUN_CONFIG.get()),
            CommandContext::Test => self.test.as_ref().or_else(|| DEFAULT_TEST_CONFIG.get()),
            CommandContext::Build => self.build.as_ref().or_else(|| DEFAULT_BUILD_CONFIG.get()),
            CommandContext::Bench => self.bench.as_ref().or_else(|| DEFAULT_BENCH_CONFIG.get()),
            CommandContext::Script => self.script.as_ref().or_else(|| DEFAULT_SCRIPT_CONFIG.get()),
        };

        command_config.and_then(|config| config.configs.get(config_name))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandConfig {
    pub default: String,
    pub configs: HashMap<String, CommandDetails>,
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
                            command_type: CommandType::Cargo,
                            command: Some(
                                "run --package ${packageName} --bin ${binaryName}".to_string(),
                            ),
                            params: Some("".to_string()),
                            allow_multiple_instances: Some(false),
                            working_directory: Some("${workspaceFolder}".to_string()),
                            pre_command: Some("".to_string()),
                            env: Some(HashMap::new()),
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
                            command_type: CommandType::Cargo,
                            command: Some("test".to_string()),
                            params: Some("".to_string()),
                            allow_multiple_instances: Some(false),
                            working_directory: Some("${workspaceFolder}".to_string()),
                            pre_command: Some("".to_string()),
                            env: Some(HashMap::new()),
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
                            command_type: CommandType::Cargo,
                            command: Some("build".to_string()),
                            params: Some("".to_string()),
                            allow_multiple_instances: Some(false),
                            working_directory: Some("${workspaceFolder}".to_string()),
                            pre_command: Some("".to_string()),
                            env: Some(HashMap::new()),
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
                            command_type: CommandType::Shell,
                            command: None,
                            params: Some("".to_string()),
                            allow_multiple_instances: Some(false),
                            working_directory: Some("${workspaceFolder}".to_string()),
                            pre_command: Some("".to_string()),
                            env: Some(HashMap::new()),
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CommandDetails {
    #[serde(rename = "type")]
    pub command_type: CommandType,
    pub command: Option<String>,
    pub params: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub allow_multiple_instances: Option<bool>,
    pub working_directory: Option<String>,
    pub pre_command: Option<String>,
}
