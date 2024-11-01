use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{CommandConfig, CommandType};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Config {
    #[serde(default)]
    pub run: Option<Vec<CommandConfig>>,
    #[serde(default)]
    pub test: Option<Vec<CommandConfig>>,
    #[serde(default)]
    pub build: Option<Vec<CommandConfig>>,
    #[serde(default)]
    pub bench: Option<Vec<CommandConfig>>,
}

impl Config {
    pub fn init() {
        let home = dirs::home_dir().expect("Could not find home directory");
        let config_dir = home.join(".cargo-runner");
        let config_path = config_dir.join("config.toml");

        // Check if config already exists
        if config_path.exists() {
            return;
        }

        // Create directory if it doesn't exist
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        // Generate default config and serialize to TOML
        let default_config = Self::default_config();
        let toml =
            toml::to_string_pretty(&default_config).expect("Failed to serialize default config");

        // Write to file
        fs::write(&config_path, toml).expect("Failed to write default config file");
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
        // Create a vector of tuples containing references to both source and destination
        let configs_to_merge = [
            (other.run.as_ref(), &mut self.run),
            (other.test.as_ref(), &mut self.test),
            (other.build.as_ref(), &mut self.build),
            (other.bench.as_ref(), &mut self.bench),
        ];

        // Process each pair of configs
        for (other_configs, base_configs) in configs_to_merge {
            if let Some(other_configs) = other_configs {
                if base_configs.is_none() {
                    *base_configs = Some(Vec::new());
                }

                let base = base_configs.as_mut().unwrap();
                for other_config in other_configs {
                    if let Some(existing) = base.iter_mut().find(|c| c.name == other_config.name) {
                        existing.merge(other_config);
                    } else {
                        base.push(other_config.clone());
                    }
                }
            }
        }
    }

    pub fn default_config() -> Self {
        Config {
            run: Some(vec![CommandConfig {
                name: "default".to_string(),
                command_type: Some(CommandType::Cargo),
                command: Some("cargo".to_string()),
                sub_command: Some("run".to_string()),
                allowed_subcommands: Some(vec![]),
                env: Some(HashMap::new()),
            }]),
            test: Some(vec![CommandConfig {
                name: "default".to_string(),
                command_type: Some(CommandType::Cargo),
                command: Some("cargo".to_string()),
                sub_command: Some("test".to_string()),
                allowed_subcommands: Some(vec![]),
                env: Some(HashMap::new()),
            }]),
            build: Some(vec![CommandConfig {
                name: "default".to_string(),
                command_type: Some(CommandType::Cargo),
                command: Some("cargo".to_string()),
                sub_command: Some("build".to_string()),
                allowed_subcommands: Some(vec![]),
                env: Some(HashMap::new()),
            }]),
            bench: Some(vec![CommandConfig {
                name: "default".to_string(),
                command_type: Some(CommandType::Cargo),
                command: Some("cargo".to_string()),
                sub_command: Some("bench".to_string()),
                allowed_subcommands: Some(vec![]),
                env: Some(HashMap::new()),
            }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_dx_config() {
        let content = fs::read_to_string("cargo-runner-dx.toml")
            .expect("Failed to read cargo-runner-dx.toml");

        let config: Config =
            toml::from_str(&content).expect("Failed to parse cargo-runner-dx.toml");

        // Verify the run config exists and contains the dx command
        let run_configs = config.run.expect("Run config should exist");
        assert_eq!(run_configs.len(), 1);

        let dx_config = &run_configs[0];
        assert_eq!(dx_config.name, "dx");
        assert_eq!(dx_config.command, Some("dx".to_string()));
        assert_eq!(dx_config.sub_command, Some("serve".to_string()));
        assert!(matches!(dx_config.command_type, Some(CommandType::Shell)));

        // Verify other configs are None
        assert!(config.test.is_none());
        assert!(config.build.is_none());
        assert!(config.bench.is_none());
    }

    #[test]
    fn test_merge_configs() {
        // Create a base config with defaults
        let mut base_config = Config::default_config();

        // Create a dx config
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

        // Merge configs
        base_config.merge(dx_config);

        // Verify merged results
        let run_configs = base_config.run.expect("Run config should exist");
        assert_eq!(run_configs.len(), 2); // Should have both default and dx

        // Find and verify dx config
        let dx_config = run_configs
            .iter()
            .find(|c| c.name == "dx")
            .expect("dx config should exist");

        assert_eq!(dx_config.command, Some("dx".to_string()));
        assert_eq!(dx_config.sub_command, Some("serve".to_string()));
        assert!(matches!(dx_config.command_type, Some(CommandType::Shell)));

        // Verify default config still exists
        let default_config = run_configs
            .iter()
            .find(|c| c.name == "default")
            .expect("default config should exist");

        assert_eq!(default_config.command, Some("cargo".to_string()));
        assert_eq!(default_config.sub_command, Some("run".to_string()));
        assert_eq!(default_config.command_type, Some(CommandType::Cargo));
    }
}
