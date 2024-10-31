use serde::{Deserialize, Serialize};

use crate::errors::ConfigError;

use super::{ContextType, CommandConfig};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Context {
    pub run: Option<CommandConfig>,
    pub test: Option<CommandConfig>,
    pub build: Option<CommandConfig>,
    pub bench: Option<CommandConfig>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            run: Some(CommandConfig::with_context(ContextType::Run)),
            test: Some(CommandConfig::with_context(ContextType::Test)),
            build: Some(CommandConfig::with_context(ContextType::Build)),
            bench: Some(CommandConfig::with_context(ContextType::Bench)),
        }
    }
}

impl Context {
    pub fn get_configs(&self, context: ContextType) -> Vec<String> {
        match context {
            ContextType::Run => self
                .run
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            ContextType::Test => self
                .test
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            ContextType::Build => self
                .build
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            ContextType::Bench => self
                .bench
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
        }
    }

    pub fn get_or_default_config(&mut self, context: ContextType) -> &mut CommandConfig {
        match context {
            ContextType::Run => self.run.get_or_insert_with(CommandConfig::default),
            ContextType::Test => self.test.get_or_insert_with(CommandConfig::default),
            ContextType::Build => self.build.get_or_insert_with(CommandConfig::default),
            ContextType::Bench => self.bench.get_or_insert_with(CommandConfig::default),
        }
    }
    pub fn set_default_config(
        &mut self,
        context: ContextType,
        new_default_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_config = match context {
            ContextType::Run => &mut self.run,
            ContextType::Test => &mut self.test,
            ContextType::Build => &mut self.build,
            ContextType::Bench => &mut self.bench,
        };

        if let Some(config) = command_config {
            if config.configs.contains_key(new_default_key) {
                config.default = new_default_key.to_string();
                Ok(())
            } else {
                Err(Box::new(ConfigError::ConfigKeyNotFound(
                    new_default_key.to_string(),
                )))
            }
        } else {
            Err(Box::new(ConfigError::ConfigKeyNotFound(
                new_default_key.to_string(),
            )))
        }
    }
}
