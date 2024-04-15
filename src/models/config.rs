use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::path::PathBuf;
use toml;

use crate::errors::ConfigError;
use crate::global::{CONFIGURATION_FILE_CONTENT, DEFAULT_CONFIG_PATH};
use crate::helpers::{read_file, write_to_config_file};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum CommandContext {
    Run,
    Test,
    Build,
    Bench,
    Script,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum CommandType {
    #[default]
    Cargo,
    Shell,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Config {
    #[serde(default = "default_commands_on_empty_file")]
    pub commands: Commands,
}

fn default_commands_on_empty_file() -> Commands {
    Commands {
        run: Some(CommandConfig::with_context("run")),
        test: Some(CommandConfig::with_context("test")),
        build: Some(CommandConfig::with_context("build")),
        bench: Some(CommandConfig::with_context("bench")),
        script: None,
    }
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Config, Box<dyn Error>> {
        if let Some(file_path) = path {
            read_file(file_path.as_path())?;
        } else {
            read_file(DEFAULT_CONFIG_PATH.get().unwrap())?;
        }

        let file_content = CONFIGURATION_FILE_CONTENT.lock().unwrap();

        let config: Config = toml::from_str(&file_content)?;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Commands {
    pub run: Option<CommandConfig>,
    pub test: Option<CommandConfig>,
    pub build: Option<CommandConfig>,
    pub bench: Option<CommandConfig>,
    pub script: Option<CommandConfig>,
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

impl Commands {
    pub fn get_configs(&self, context: CommandContext) -> Vec<String> {
        match context {
            CommandContext::Run => self
                .run
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CommandContext::Test => self
                .test
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CommandContext::Build => self
                .build
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CommandContext::Bench => self
                .bench
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CommandContext::Script => self
                .script
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
        }
    }

    pub fn get_or_default_config(&mut self, context: CommandContext) -> &mut CommandConfig {
        match context {
            CommandContext::Run => self.run.get_or_insert_with(CommandConfig::default),
            CommandContext::Test => self.test.get_or_insert_with(CommandConfig::default),
            CommandContext::Build => self.build.get_or_insert_with(CommandConfig::default),
            CommandContext::Bench => self.bench.get_or_insert_with(CommandConfig::default),
            CommandContext::Script => self.script.get_or_insert_with(CommandConfig::default),
        }
    }
    pub fn set_default_config(
        &mut self,
        context: CommandContext,
        new_default_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_config = match context {
            CommandContext::Run => &mut self.run,
            CommandContext::Test => &mut self.test,
            CommandContext::Build => &mut self.build,
            CommandContext::Bench => &mut self.bench,
            CommandContext::Script => &mut self.script,
        };

        if let Some(config) = command_config {
            if config.configs.contains_key(new_default_key) {
                config.default = new_default_key.to_string();
                Ok(())
            } else {
                Err(Box::new(ConfigError::ConfigKey(
                    new_default_key.to_string(),
                )))
            }
        } else {
            Err(Box::new(ConfigError::ConfigKey(
                new_default_key.to_string(),
            )))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CommandConfig {
    #[serde(default = "default_command_config")]
    pub default: String,
    pub configs: HashMap<String, CommandDetails>,
}

fn default_command_config() -> String {
    String::from("default")
}

impl CommandConfig {
    fn default_command_details(command: &str, command_type: CommandType) -> CommandDetails {
        CommandDetails {
            command_type,
            command: command.to_string(),
            params: "".to_string(),
            allow_multiple_instances: false,
            working_directory: "${workspaceFolder}".to_string(),
            pre_command: BTreeSet::new(),
            env: HashMap::new(),
        }
    }

    pub fn with_context(context: &str) -> Self {
        let default_details = match context {
            "run" => Self::default_command_details(
                "run --package ${packageName} --bin ${binaryName}",
                CommandType::Cargo,
            ),
            "test" => Self::default_command_details("test", CommandType::Cargo),
            "build" => Self::default_command_details("build", CommandType::Cargo),
            "bench" => Self::default_command_details("bench", CommandType::Cargo),
            _ => Self::default_command_details("script", CommandType::Shell),
        };

        let mut configs = HashMap::new();
        configs.insert("default".to_string(), default_details);

        Self {
            default: "default".into(),
            configs,
        }
    }

    pub fn update_config(&mut self, key: &str, details: CommandDetails) {
        self.configs.insert(key.to_string(), details);
    }

    pub fn remove_config(&mut self, key: &str) {
        // Remove the specified config key
        self.configs.remove(key);

        // Check if the removed key was the default and reset the default if necessary
        if self.default == key {
            // Reset to a predefined fallback default key
            // Adjust this logic based on how you want to handle resetting the default
            // For example, you could check for other existing keys and choose one of them as the new default
            self.default = "default".to_string(); // Assuming "default" is a sensible fallback default key

            // Alternatively, find the first available key in `self.configs` to set as new default
            // if you prefer dynamically choosing a new default based on existing keys
            /*
            self.default = self.configs.keys().next().cloned().unwrap_or_else(|| "default".to_string());
            */
        }
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

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct CommandDetails {
    #[serde(rename = "type", default = "default_command_type")]
    pub command_type: CommandType,
    #[serde(default = "default_command")]
    pub command: String,
    #[serde(default = "default_params")]
    pub params: String,
    #[serde(serialize_with = "serialize_env", default = "default_env")]
    pub env: HashMap<String, String>,
    #[serde(default = "default_allow_multiple_instances")]
    pub allow_multiple_instances: bool,
    #[serde(default = "default_working_directory")]
    pub working_directory: String,
    #[serde(default = "default_pre_command")]
    pub pre_command: BTreeSet<String>,
}

fn default_command_type() -> CommandType {
    CommandType::Cargo
}

fn default_command() -> String {
    String::from("run")
}

fn default_params() -> String {
    String::new()
}

fn default_env() -> HashMap<String, String> {
    HashMap::new()
}

fn default_allow_multiple_instances() -> bool {
    false
}
fn default_working_directory() -> String {
    String::from("${workspaceFolder}")
}
fn default_pre_command() -> BTreeSet<String> {
    BTreeSet::new()
}

fn serialize_env<S>(env: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(env.len()))?;
    for (k, v) in env {
        if let Ok(bool_val) = v.parse::<bool>() {
            map.serialize_entry(k, &bool_val)?;
        } else if let Ok(int_val) = v.parse::<i64>() {
            map.serialize_entry(k, &int_val)?;
        } else {
            // Fallback to string if it's neither bool nor int
            map.serialize_entry(k, v)?;
        }
    }
    map.end()
}
