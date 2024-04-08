use std::collections::{BTreeSet, HashMap};

use crate::{
    config::{CommandDetails, CommandType},
    errors::ConfigError,
    validator::ValidateCommandDetails,
};

#[derive(Default)]
pub struct CommandDetailsBuilder {
    command_type: CommandType,
    command: String,
    params: String,
    env: HashMap<String, String>,
    allow_multiple_instances: bool,
    working_directory: String,
    pre_command: BTreeSet<String>,
    validators: Vec<Box<dyn ValidateCommandDetails>>,
}

impl CommandDetailsBuilder {
    pub fn add_validator<T: ValidateCommandDetails + 'static>(mut self, validator: T) -> Self {
        self.validators.push(Box::new(validator));
        self
    }
    pub fn new(command_type: CommandType, command: &str) -> Self {
        Self {
            command_type,
            command: command.to_string(),
            working_directory: "${workspaceFolder}".to_string(),
            ..Default::default()
        }
    }

    pub fn command_type(mut self, command_type: CommandType) -> Self {
        self.command_type = command_type;
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

    pub fn build(self) -> Result<CommandDetails, ConfigError> {
        let command_details = CommandDetails {
            command_type: self.command_type,
            command: self.command,
            params: self.params,
            env: self.env,
            allow_multiple_instances: self.allow_multiple_instances,
            working_directory: self.working_directory,
            pre_command: self.pre_command,
        };

        for validator in self.validators {
            validator.validate(&command_details)?;
        }

        Ok(command_details)
    }
}
