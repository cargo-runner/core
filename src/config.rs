use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub commands: Commands,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commands {
    pub run: CommandConfig,
    pub test: CommandConfig,
    pub build: CommandConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommandConfig {
    pub default: String,
    pub configs: HashMap<String, CommandDetails>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommandDetails {
    #[serde(rename = "type")]
    pub command_type: String,
    pub command: String,
    pub params: String,
    pub env: HashMap<String, String>,
    pub allow_multiple_instances: bool,
    pub working_directory: String,
    pub pre_command: String,
}

impl Default for Commands {
    fn default() -> Self {
        let mut run_configs = HashMap::new();
        run_configs.insert(
            "default".into(),
            CommandDetails {
                command_type: "cargo".into(),
                command: "run".into(),
                params: "".into(),
                env: HashMap::new(),
                allow_multiple_instances: false,
                working_directory: ".".into(),
                pre_command: "".into(),
            },
        );

        let mut test_configs = HashMap::new();
        test_configs.insert(
            "default".into(),
            CommandDetails {
                command_type: "cargo".into(),
                command: "test".into(),
                params: "".into(),
                env: HashMap::new(),
                allow_multiple_instances: true,
                working_directory: ".".into(),
                pre_command: "".into(),
            },
        );

        let mut build_configs = HashMap::new();
        build_configs.insert(
            "default".into(),
            CommandDetails {
                command_type: "cargo".into(),
                command: "build".into(),
                params: "".into(),
                env: HashMap::new(),
                allow_multiple_instances: false,
                working_directory: ".".into(),
                pre_command: "".into(),
            },
        );

        Commands {
            run: CommandConfig {
                default: "standard".into(),
                configs: run_configs,
            },
            test: CommandConfig {
                default: "default".into(),
                configs: test_configs,
            },
            build: CommandConfig {
                default: "debug".into(),
                configs: build_configs,
            },
        }
    }
}

