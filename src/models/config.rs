use std::error::Error;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml;

use crate::global::{CONFIGURATION_FILE_CONTENT, DEFAULT_CONFIG_PATH};
use crate::helpers::{read_file, write_to_config_file};

use super::{ContextType, CommandConfig, Context};


#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Config {
    #[serde(default = "default_config_on_empty_file")]
    pub context: Context,
}

fn default_config_on_empty_file() -> Context {
    Context {
        run: Some(CommandConfig::with_context(ContextType::Run)),
        test: Some(CommandConfig::with_context(ContextType::Test)),
        build: Some(CommandConfig::with_context(ContextType::Build)),
        bench: Some(CommandConfig::with_context(ContextType::Bench)),
    }
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Config, Box<dyn Error>> {
        if let Some(file_path) = path {
            read_file(file_path.as_path())?;
        } else {
            read_file(DEFAULT_CONFIG_PATH.get().unwrap())?;
        }

        let file_content = CONFIGURATION_FILE_CONTENT.lock().unwrap();

        let config: Config = toml::from_str(&file_content)?;

        Ok(config)
    }

    pub fn save(&self, path: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
        // Determine the file path to use: provided path or default
        let file_path = path.unwrap_or_else(|| {
            DEFAULT_CONFIG_PATH
                .get()
                .expect("DEFAULT_CONFIG_PATH not set")
                .clone()
        });

        // We need Config Struct and all Other Fields (struct or enum) to be impl Serialize
        let toml_string = toml::to_string_pretty(&self)?;

        // Write the serialized string to the file line by line
        write_to_config_file(&file_path, &toml_string)?;

        Ok(())
    }
}

