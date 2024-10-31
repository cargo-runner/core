use std::collections::{BTreeSet, HashMap};

use crate::{
    errors::ConfigError,
    models::{CargoContext, CommandDetails, CommandType},
    validator::ValidateCommandDetails,
};

#[derive(Default)]
pub struct CargoConfigBuilder {
    command_type: CommandType,
    env: HashMap<String, String>,
    command: String,
    toolchain: Option<String>,
    sub_command: String,
    sub_action: String,
    params: String,
    target: Option<String>,
    features: BTreeSet<String>,
    allowed_subcommands: BTreeSet<String>,
    validators: Vec<Box<dyn ValidateCommandDetails>>,
}

impl CargoConfigBuilder {
    pub fn add_validator<T: ValidateCommandDetails + 'static>(mut self, validator: T) -> Self {
        self.validators.push(Box::new(validator));
        self
    }

    pub fn new(command_type: CommandType, context: CargoContext) -> Self {
        match context {
            CargoContext::Run => Self {
                command_type: command_type.clone(),
                command: command_type.into(),
                sub_command: CargoContext::Run.sub_command(),
                ..Default::default()
            },
            CargoContext::Test => Self {
                command_type: command_type.clone(),
                command: command_type.into(),
                sub_command: CargoContext::Test.sub_command(),
                ..Default::default()
            },
            CargoContext::Build => Self {
                command_type: command_type.clone(),
                command: command_type.into(),
                sub_command: CargoContext::Build.sub_command(),
                ..Default::default()
            },
            CargoContext::Bench => Self {
                command_type: command_type.clone(),
                command: command_type.into(),
                sub_command: CargoContext::Bench.sub_command(),
                ..Default::default()
            },
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

    pub fn sub_action(mut self, sub_action: &str) -> Self {
        self.sub_action = sub_action.to_string();
        self
    }

    pub fn sub_command(mut self, sub_command: &str) -> Self {
        self.sub_command = sub_command.to_string();
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


    pub fn allowed_subcommands(mut self, allowed_subcommands: BTreeSet<String>) -> Self {
        self.allowed_subcommands = allowed_subcommands;
        self
    }

    pub fn toolchain(mut self, toolchain: Option<String>) -> Self {
        self.toolchain = toolchain;
        self
    }

    pub fn features(mut self, features: BTreeSet<String>) -> Self {
      self.features =  features;
      self
    }

    pub fn target(mut self, target: Option<String>) -> Self {
        self.target = target;
        self
    }

    pub fn build(self) -> Result<CommandDetails, ConfigError> {
        let command_details = CommandDetails {
            command_type: self.command_type,
            env: self.env,
            command: self.command,
            toolchain: self.toolchain,
            sub_command: self.sub_command,
            sub_action: self.sub_action,
            params: self.params,
            target: self.target,
            features: self.features,
            allowed_subcommands: self.allowed_subcommands,
        };

        for validator in self.validators {
            validator.validate(&command_details)?;
        }

        Ok(command_details)
    }
}
