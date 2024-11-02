use std::{collections::HashMap, fs,  path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{CommandConfig, CommandType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Config (
    HashMap<String, Option<Vec<CommandConfig>>>,
);

impl Default for Config {
    fn default() -> Self {
        let mut commands = HashMap::new();
        
        // Add default commands
        commands.insert("run".to_string(), Some(vec![CommandConfig {
            name: "default".to_string(),
            command_type: Some(CommandType::Cargo),
            command: Some("cargo".to_string()),
            sub_command: Some("run".to_string()),
            allowed_subcommands: Some(vec![]),
            env: Some(HashMap::new()),
        }]));

        commands.insert("test".to_string(), Some(vec![CommandConfig {
            name: "default".to_string(),
            command_type: Some(CommandType::Cargo),
            command: Some("cargo".to_string()),
            sub_command: Some("test".to_string()),
            allowed_subcommands: Some(vec![]),
            env: Some(HashMap::new()),
        }]));

        commands.insert("build".to_string(), Some(vec![CommandConfig {
            name: "default".to_string(),
            command_type: Some(CommandType::Cargo),
            command: Some("cargo".to_string()),
            sub_command: Some("build".to_string()),
            allowed_subcommands: Some(vec![]),
            env: Some(HashMap::new()),
        }]));

        commands.insert("bench".to_string(), Some(vec![CommandConfig {
            name: "default".to_string(),
            command_type: Some(CommandType::Cargo),
            command: Some("cargo".to_string()),
            sub_command: Some("bench".to_string()),
            allowed_subcommands: Some(vec![]),
            env: Some(HashMap::new()),
        }]));

        Config(commands)
    }
}

impl Into<String> for Config {
    fn into(self) -> String {
        toml::to_string_pretty(&self).expect("Failed to serialize config to TOML")
    }
}

impl From<String> for Config {
    fn from(value: String) -> Self {
        toml::from_str(&value).expect("Failed to convert String to Config")
    }
}

impl From<&str> for Config {
    fn from(value: &str) -> Self {
        toml::from_str(value).expect("Failed to convert String to Config")
    }
}

impl Config {
    pub fn init() -> Option<Config> {
        let home = dirs::home_dir().expect("Could not find home directory");
        let config_dir = home.join(".cargo-runner");
        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            return Some(Config::load(config_path));
        }

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let toml = toml::to_string_pretty(&Self::default())
            .expect("Failed to serialize default config");
        fs::write(&config_path, toml).expect("Failed to write default config file");

        None
    }

    pub fn load(path: PathBuf) -> Self {
        let data = fs::read_to_string(&path)
            .expect(&format!("Failed to read config path: {}", path.display()));
        toml::from_str(&data).expect(&format!(
            "failed to parse config file from: {}",
            path.display()
        ))
    }
    pub fn merge(&mut self, other: Config) {
        for (command_type, other_configs) in other.0 {
            if let Some(other_configs) = other_configs {
                let base_configs = self.0
                    .entry(command_type)
                    .or_insert_with(|| Some(Vec::new()));
                
                if let Some(base) = base_configs {
                    for other_config in other_configs {
                        if let Some(existing) = base.iter_mut().find(|c| c.name == other_config.name) {
                            existing.merge(&other_config);
                        } else {
                            base.push(other_config.clone());
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dx_config() {
        let dx_content = r#"
        [[run]]
        name = "dx"
        command_type = "shell"
        command = "dx"
        sub_command = "serve"
        allowed_subcommands = ["build", "serve"]
        [run.env]
        "#;

        let config: Config = toml::from_str(dx_content).expect("Failed to parse dx config");

        let run_configs = config.0.get("run")
            .expect("Run config should exist")
            .as_ref()
            .expect("Run config should have values");
            
        assert_eq!(run_configs.len(), 1);

        let dx_config = &run_configs[0];
        assert_eq!(dx_config.name, "dx");
        assert_eq!(dx_config.command, Some("dx".to_string()));
        assert_eq!(dx_config.sub_command, Some("serve".to_string()));
        assert!(matches!(dx_config.command_type, Some(CommandType::Shell)));

        assert!(config.0.get("test").is_none());
        assert!(config.0.get("build").is_none());
        assert!(config.0.get("bench").is_none());
    }

    #[test]
    fn test_merge_configs() {
        let mut base_config = Config::default();

        let dx_content = r#"
        [[run]]
        name = "dx"
        command_type = "shell"
        command = "dx"
        sub_command = "serve"
        allowed_subcommands = ["build", "serve"]
        [run.env]
        "#;

        let dx_config: Config = toml::from_str(dx_content).expect("Failed to parse dx config");

        base_config.merge(dx_config);

        let run_configs = base_config.0.get("run")
            .expect("Run config should exist")
            .as_ref()
            .expect("Run config should have values");
            
        assert_eq!(run_configs.len(), 2);

        let dx_config = run_configs
            .iter()
            .find(|c| c.name == "dx")
            .expect("dx config should exist");

        assert_eq!(dx_config.command, Some("dx".to_string()));
        assert_eq!(dx_config.sub_command, Some("serve".to_string()));
        assert!(matches!(dx_config.command_type, Some(CommandType::Shell)));

        let default_config = run_configs
            .iter()
            .find(|c| c.name == "default")
            .expect("default config should exist");

        assert_eq!(default_config.command, Some("cargo".to_string()));
        assert_eq!(default_config.sub_command, Some("run".to_string()));
        assert_eq!(default_config.command_type, Some(CommandType::Cargo));
    }
}