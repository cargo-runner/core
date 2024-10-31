use serde::{Deserialize, Serialize};

use crate::errors::ConfigError;

use super::{CargoContext, CommandConfig};

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
            run: Some(CommandConfig::with_context(CargoContext::Run)),
            test: Some(CommandConfig::with_context(CargoContext::Test)),
            build: Some(CommandConfig::with_context(CargoContext::Build)),
            bench: Some(CommandConfig::with_context(CargoContext::Bench)),
        }
    }
}

impl Context {
    pub fn get_configs(&self, context: CargoContext) -> Vec<String> {
        match context {
            CargoContext::Run => self
                .run
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CargoContext::Test => self
                .test
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CargoContext::Build => self
                .build
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
            CargoContext::Bench => self
                .bench
                .as_ref()
                .map_or(vec![], |config| config.configs.keys().cloned().collect()),
        }
    }

    pub fn get_or_default_config(&mut self, context: CargoContext) -> &mut CommandConfig {
        match context {
            CargoContext::Run => self.run.get_or_insert_with(CommandConfig::default),
            CargoContext::Test => self.test.get_or_insert_with(CommandConfig::default),
            CargoContext::Build => self.build.get_or_insert_with(CommandConfig::default),
            CargoContext::Bench => self.bench.get_or_insert_with(CommandConfig::default),
        }
    }
    pub fn set_default_config(
        &mut self,
        context: CargoContext,
        new_default_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command_config = match context {
            CargoContext::Run => &mut self.run,
            CargoContext::Test => &mut self.test,
            CargoContext::Build => &mut self.build,
            CargoContext::Bench => &mut self.bench,
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
