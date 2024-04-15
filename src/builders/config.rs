use std::collections::{BTreeSet, HashMap};

use crate::{
    errors::ConfigError,
    models::config::{CommandContext, CommandDetails, CommandType, Config},
    validator::Validate,
};

#[derive(Default)]
pub struct ConfigBuilder {
    pub config_key: String,
    pub command_type: CommandType,
    pub command: String,
    pub params: String,
    pub env: HashMap<String, String>,
    pub allow_multiple_instances: bool,
    pub working_directory: String,
    pub pre_command: BTreeSet<String>,
    validators: Vec<Validate>,
}

impl ConfigBuilder {
    pub fn add_validator(mut self, validator: Validate) -> Self {
        self.validators.push(validator);
        self
    }

    pub fn from(context: CommandContext, config: &mut Config, config_key: &str) -> Self {
        let command = config.commands.get_or_default_config(context);
        let config = command
            .configs
            .get(config_key).unwrap_or(&CommandDetails::default()).clone();
            
            return Self {
                config_key: config_key.to_string(),
                command_type: config.command_type,
                command: config.command,
                params: config.params,
                env: config.env,
                allow_multiple_instances: config.allow_multiple_instances,
                working_directory: config.working_directory,
                pre_command: config.pre_command,
                validators: vec![],
            };
        //Self::new(context)
    }

    pub fn new(context: CommandContext) -> Self {
        match context {
            CommandContext::Run => Self {
                command_type: CommandType::Cargo,
                command: String::from("run --package ${packageName} --bin ${binaryName}"),
                working_directory: "${workspaceFolder}".to_string(),
                ..Default::default()
            },
            CommandContext::Test => Self {
                command_type: CommandType::Cargo,
                command: String::from("test"),
                working_directory: "${workspaceFolder}".to_string(),
                ..Default::default()
            },
            CommandContext::Build => Self {
                command_type: CommandType::Cargo,
                command: String::from("build"),
                working_directory: "${workspaceFolder}".to_string(),
                ..Default::default()
            },
            CommandContext::Bench => Self {
                command_type: CommandType::Cargo,
                command: String::from("bench"),
                working_directory: "${workspaceFolder}".to_string(),
                ..Default::default()
            },
            CommandContext::Script => Self {
                command_type: CommandType::Shell,
                working_directory: "${workspaceFolder}".to_string(),
                ..Default::default()
            },
        }
    }

    pub fn command_type(mut self, command_type: CommandType) -> Self {
        self.command_type = command_type;
        self
    }

    pub fn config_key(mut self, config_key: &str) -> Self {
        self.config_key = config_key.to_string();
        self
    }

    pub fn command(mut self, command: &str) -> Self {
        self.command = command.to_string();
        self
    }

    pub fn params(mut self, params: &str) -> Self {
        self.params = params.to_string();
        self
    }

    pub fn env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub fn allow_multiple_instances(mut self, allow_multiple_instances: bool) -> Self {
        self.allow_multiple_instances = allow_multiple_instances;
        self
    }

    pub fn working_directory(mut self, working_directory: &str) -> Self {
        self.working_directory = working_directory.to_string();
        self
    }

    pub fn pre_command(mut self, pre_command: BTreeSet<String>) -> Self {
        self.pre_command = pre_command;
        self
    }

    pub fn build(self, config: &Config) -> Result<CommandDetails, ConfigError> {
        for validator in &self.validators {
            validator.validate(&config, &self)?;
        }

        let command_details = CommandDetails {
            command_type: self.command_type,
            command: self.command,
            params: self.params,
            env: self.env,
            allow_multiple_instances: self.allow_multiple_instances,
            working_directory: self.working_directory,
            pre_command: self.pre_command,
        };

        Ok(command_details)
    }
}
