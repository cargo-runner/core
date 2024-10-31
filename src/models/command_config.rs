use std::collections::HashMap;

use serde::{Deserialize, Serialize};


use super::{CargoContext, CommandDetails, CommandType};


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
    fn default_command_details(
        command_type: CommandType,
        command: String,
        sub_command: String,
        sub_action: String,
        params: String,
    ) -> CommandDetails {
        CommandDetails {
            command_type,
            command,
            sub_command,
            sub_action: sub_action,
            params: params,
            env: HashMap::new(),
            ..Default::default()
        }
    }

    pub fn with_context(context: CargoContext) -> Self {
        let default_details = match context {
            CargoContext::Run => Self::default_command_details(
                CommandType::Cargo,
                String::from("cargo"),
                context.sub_command(),
                String::new(),
                String::new(),
            ),
            CargoContext::Test => Self::default_command_details(
                CommandType::Cargo,
                String::from("cargo"),
                context.sub_command(),
                String::new(),
                String::new(),
            ),
            CargoContext::Build => Self::default_command_details(
                CommandType::Cargo,
                String::from("cargo"),
                context.sub_command(),
                String::new(),
                String::new(),
            ),
            CargoContext::Bench => Self::default_command_details(
                CommandType::Cargo,
                String::from("cargo"),
                context.sub_command(),
                String::new(),
                String::new(),
            ),
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
