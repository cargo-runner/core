use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub commands: Commands,
}

#[derive(Debug, Deserialize, Default)]
pub struct Commands {
    pub run: Option<CommandConfig>,
    pub test: Option<CommandConfig>,
    pub build: Option<CommandConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct CommandConfig {
    pub default: Option<String>,
    #[serde(default)]
    pub configs: HashMap<String, CommandDetails>,
}

#[derive(Debug, Deserialize, Default)]
pub struct CommandDetails {
    #[serde(rename = "type", default)]
    pub command_type: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub params: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default = "default_bool_false")]
    pub allow_multiple_instances: bool,
    #[serde(default)]
    pub working_directory: Option<String>,
    #[serde(default)]
    pub pre_command: Option<String>,
}

fn default_bool_false() -> bool {
    false
}
