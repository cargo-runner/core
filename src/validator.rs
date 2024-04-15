use std::collections::{BTreeSet, HashMap};

use crate::{
    builders::config::ConfigBuilder,
    errors::ConfigError,
    models::config::{CommandContext, CommandType, Config},
};

pub enum Validate {
    CommandType(fn(&Config, &CommandType) -> Result<(), ConfigError>),
    Command(fn(&Config, &str) -> Result<(), ConfigError>),
    Params(fn(&Config, &str) -> Result<(), ConfigError>),
    Env(fn(&Config, &HashMap<String, String>) -> Result<(), ConfigError>),
    AllowMultipleInstances(fn(&Config, &bool) -> Result<(), ConfigError>),
    WorkingDirectory(fn(&Config, &str) -> Result<(), ConfigError>),
    PreCommand(fn(&Config, &str, &BTreeSet<String>) -> Result<(), ConfigError>),
}

impl Validate {
    pub fn validate(&self, config: &Config, builder: &ConfigBuilder) -> Result<(), ConfigError> {
        match self {
            Validate::CommandType(validator) => validator(&config, &builder.command_type),
            Validate::Command(validator) => validator(&config, &builder.command),
            Validate::Params(validator) => validator(&config, &builder.params),
            Validate::Env(validator) => validator(&config, &builder.env),
            Validate::AllowMultipleInstances(validator) => {
                validator(&config, &builder.allow_multiple_instances)
            }
            Validate::WorkingDirectory(validator) => validator(&config, &builder.working_directory),
            Validate::PreCommand(validator) => {
                validator(&config, &builder.config_key, &builder.pre_command)
            }
        }
    }
}

pub fn pre_command_validator() -> Validate {
    Validate::PreCommand(|config, config_key: &str, pre_command| {
        let valid_pre_command_keys = config.commands.get_configs(CommandContext::Run);

        if pre_command.contains(config_key) {
            return Err(ConfigError::PreCommand(format!(
                "You cannot use {} as a pre_command",
                config_key
            )));
        }

        for command in pre_command {
            if !valid_pre_command_keys.contains(command) {
                return Err(ConfigError::PreCommand(format!(
                    "Pre-command must be any of the following: [{}]",
                    valid_pre_command_keys.join(",")
                )));
            }
        }
        Ok(())
    })
}

pub fn command_validator() -> Validate {
    Validate::Command(|_, cmd| {
        if cmd.is_empty() {
            Err(ConfigError::ConfigKey(
                "Command cannot be empty".to_string(),
            ))
        } else {
            Ok(())
        }
    })
}

pub fn command_type_validator() -> Validate {
    Validate::CommandType(|_, command_type| {
        if *command_type == CommandType::Cargo {
            return Ok(());
        }
        Err(ConfigError::CommandType(
            "Command Type Error".to_owned(),
        ))
    })
}
