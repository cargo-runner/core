use std::collections::HashMap;

use crate::config::{CommandDetails, CommandType};

#[derive(Debug, Clone, Default)]
pub struct CommandDetailsBuilder {
    command_type: CommandType,
    command: String,
    params: Option<String>,
    env: Option<HashMap<String, String>>,
    allow_multiple_instances: Option<bool>,
    working_directory: Option<String>,
    pre_command: Option<String>,
}

impl CommandDetailsBuilder {
    pub fn new(command_type: CommandType, command: &str) -> Self {
        Self {
            command_type,
            command: command.to_string(),
            params: Some("".to_string()),
            env: Some(HashMap::new()),
            allow_multiple_instances: Some(false),
            working_directory: Some("${workspaceFolder}".to_string()),
            pre_command: Some("".to_string()),
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
        self.params = Some(params.to_string());
        self
    }

    pub fn env(mut self, env: HashMap<String, String>) -> Self {
        self.env = Some(env);
        self
    }

    pub fn allow_multiple_instances(mut self, allow_multiple_instances: bool) -> Self {
        self.allow_multiple_instances = Some(allow_multiple_instances);
        self
    }

    pub fn working_directory(mut self, working_directory: &str) -> Self {
        self.working_directory = Some(working_directory.to_string());
        self
    }

    pub fn pre_command(mut self, pre_command: &str) -> Self {
        self.pre_command = Some(pre_command.to_string());
        self
    }

    // Finalize and construct a CommandDetails instance
    pub fn build(self) -> CommandDetails {
        CommandDetails {
            command_type: self.command_type,
            command: Some(self.command),
            params: self.params,
            env: self.env,
            allow_multiple_instances: self.allow_multiple_instances,
            working_directory: self.working_directory,
            pre_command: self.pre_command,
        }
    }
}
