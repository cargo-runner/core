use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{CommandType, Config, Context};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoRunner(pub HashMap<String, (Option<String>, Option<Vec<Config>>)>);

impl Serialize for CargoRunner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (key, (default, commands)) in &self.0 {
            #[derive(Serialize)]
            struct CommandEntry<'a> {
                default: Option<&'a String>,
                config: Option<&'a Vec<Config>>,
            }

            map.serialize_entry(
                key,
                &CommandEntry {
                    default: default.as_ref(),
                    config: commands.as_ref(),
                },
            )?;
        }

        map.end()
    }
}

// Custom deserialization implementation
impl<'de> Deserialize<'de> for CargoRunner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CommandEntry {
            default: Option<String>,
            config: Option<Vec<Config>>,
        }

        let map = HashMap::<String, CommandEntry>::deserialize(deserializer)?;

        let converted = map
            .into_iter()
            .map(|(k, v)| (k, (v.default, v.config)))
            .collect();

        Ok(CargoRunner(converted))
    }
}

impl Default for CargoRunner {
    fn default() -> Self {
        let mut commands = HashMap::new();

        // Add default commands
        commands.insert(
            "run".to_string(),
            (
                Some("default".to_string()),
                Some(vec![Config {
                    name: "default".to_string(),
                    command_type: Some(CommandType::Cargo),
                    command: Some("cargo".to_string()),
                    sub_command: Some("run".to_string()),
                    allowed_subcommands: Some(vec![]),
                    env: Some(HashMap::new()),
                }]),
            ),
        );

        commands.insert(
            "test".to_string(),
            (
                Some("default".to_string()),
                Some(vec![Config {
                    name: "default".to_string(),
                    command_type: Some(CommandType::Cargo),
                    command: Some("cargo".to_string()),
                    sub_command: Some("test".to_string()),
                    allowed_subcommands: Some(vec![]),
                    env: Some(HashMap::new()),
                }]),
            ),
        );

        commands.insert(
            "build".to_string(),
            (
                Some("default".to_string()),
                Some(vec![Config {
                    name: "default".to_string(),
                    command_type: Some(CommandType::Cargo),
                    command: Some("cargo".to_string()),
                    sub_command: Some("build".to_string()),
                    allowed_subcommands: Some(vec![]),
                    env: Some(HashMap::new()),
                }]),
            ),
        );

        commands.insert(
            "bench".to_string(),
            (
                Some("default".to_string()),
                Some(vec![Config {
                    name: "default".to_string(),
                    command_type: Some(CommandType::Cargo),
                    command: Some("cargo".to_string()),
                    sub_command: Some("bench".to_string()),
                    allowed_subcommands: Some(vec![]),
                    env: Some(HashMap::new()),
                }]),
            ),
        );

        CargoRunner(commands)
    }
}

impl Into<String> for CargoRunner {
    fn into(self) -> String {
        toml::to_string_pretty(&self).expect("Failed to serialize config to TOML")
    }
}

impl From<String> for CargoRunner {
    fn from(value: String) -> Self {
        toml::from_str(&value).expect("Failed to convert String to Config")
    }
}

impl From<&str> for CargoRunner {
    fn from(value: &str) -> Self {
        toml::from_str(value).expect("Failed to convert String to Config")
    }
}

impl CargoRunner {
    pub fn set_default(&mut self, context: Context, name: &str) -> Result<(), String> {
        if let Some((_, configs)) = self.0.get(context.into()) {
            if let Some(configs) = configs {
                if configs.iter().any(|c| c.name == name) {
                    self.0.insert(
                        context.into(),
                        (Some(name.to_string()), Some(configs.clone())),
                    );
                    return Ok(());
                }
            }
        }
        Err(format!(
            "Command '{}' not found for type '{}'",
            name, context
        ))
    }

    pub fn get_default(&self, context: Context) -> Option<&str> {
        self.0
            .get(context.into())
            .and_then(|(default, _)| default.as_ref())
            .map(|s| s.as_str())
    }

    pub fn pluck(&self, config_name: &str) -> CargoRunner {
        let mut found_configs = HashMap::new();

        for (context, (_, configs)) in &self.0 {
            if let Some(configs_vec) = configs {
                let matching_configs: Vec<Config> = configs_vec
                    .iter()
                    .filter(|c| c.name == config_name)
                    .cloned() // Clone the Config as we return owned values
                    .collect();

                if !matching_configs.is_empty() {
                    // Set the default to the config_name since it's now relevant
                    found_configs.insert(
                        context.clone(),
                        (Some(config_name.to_string()), Some(matching_configs)),
                    );
                }
            }
        }

        CargoRunner(found_configs)
    }

    pub fn find(&self, context: Context, config_name: &str) -> Option<&Config> {
        self.0.get(context.into()).and_then(|(_, configs)| {
            configs
                .as_ref()
                .and_then(|configs_vec| configs_vec.iter().find(|c| c.name == config_name))
        })
    }

    pub fn init() -> CargoRunner {
        let home = dirs::home_dir().expect("Could not find home directory");
        let config_dir = home.join(".cargo-runner");
        let config_path = Self::get_default_config_path();

        // Create the config directory if it doesn't exist
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        // Attempt to load the config
        CargoRunner::load(config_path)
    }

    pub fn reset() {
        let config_path = Self::get_default_config_path();

        let default_config = Self::default();

        Self::create_backup(&config_path);

        fs::write(
            &config_path,
            toml::to_string_pretty(&default_config).expect("Failed to serialize default config"),
        )
        .expect("Failed to write default config file");
    }

    pub async fn download(
        url: &str,
        save_path: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Download the content from the URL asynchronously
        let response = reqwest::get(url).await?;
        let content = response.text().await?;

        // Parse the fetched content as TOML into CargoRunner
        let config: CargoRunner = toml::from_str(&content)?;

        if let Some(path) = save_path {
            // Save the parsed configuration to the specified path
            fs::create_dir_all(path.parent().unwrap())?;
            let toml_content = toml::to_string_pretty(&config)?;
            fs::write(&path, toml_content)?;
        } else {
            let mut default_config = Self::default();
            default_config.merge(config.clone());

            let config_path = Self::get_default_config_path();
            // Save the parsed configuration to the default path
            Self::create_backup(&config_path);

            let toml = toml::to_string_pretty(&default_config)
                .expect("Failed to serialize default config");
            fs::write(config_path, toml).expect("Failed to write default config file");
        }

        Ok(())
    }

    fn get_default_config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".cargo-runner")
            .join("config.toml")
    }

    pub fn load(path: PathBuf) -> CargoRunner {
        match fs::read_to_string(&path) {
            Ok(data) => {
                toml::from_str(&data).unwrap_or_else(|_| {
                    eprintln!("Failed to parse config file from: {}", path.display());
                    Self::create_backup(&path); // Create a backup on parse failure

                    // Write the default config to the file
                    let default_config = Self::default();
                    let toml = toml::to_string_pretty(&default_config)
                        .expect("Failed to serialize default config");
                    fs::write(&path, toml).expect("Failed to write default config file");

                    default_config // Return default config
                })
            }
            Err(_) => {
                eprintln!("Failed to read config path: {}", path.display());
                Self::create_backup(&path); // Create a backup on read failure

                // Write the default config to the file
                let default_config = Self::default();
                let toml = toml::to_string_pretty(&default_config)
                    .expect("Failed to serialize default config");
                fs::write(&path, toml).expect("Failed to write default config file");

                default_config // Return default config
            }
        }
    }

    pub fn merge(&mut self, other: CargoRunner) {
        for (command_type, (other_default, other_configs)) in other.0 {
            let command_type_clone = command_type.clone(); // Clone command_type for later use
            let (base_default, base_configs) = self
                .0
                .entry(command_type_clone) // Use the cloned value here
                .or_insert_with(|| (None, Some(Vec::new())));

            // Merge command configurations first
            if let Some(other_configs) = other_configs {
                if let Some(base) = base_configs {
                    for other_config in other_configs {
                        // Check if the command already exists
                        if let Some(existing) =
                            base.iter_mut().find(|c| c.name == other_config.name)
                        {
                            existing.merge(&other_config); // Merge existing command
                        } else {
                            base.push(other_config.clone()); // Add new command
                        }
                    }
                }
            }

            // Now update the default value if the other configuration has one
            if let Some(ref new_default) = other_default {
                // Check if the new default exists in the command list
                if let Some(base) = base_configs {
                    if base.iter().any(|cmd| cmd.name == *new_default) {
                        *base_default = Some(new_default.clone()); // Update the default field
                    } else {
                        eprintln!(
                            "Warning: Default command '{}' does not exist in the '{}' commands.",
                            new_default, command_type
                        );
                        // Optionally, you can set it to None or keep the existing default
                    }
                }
            }
        }
    }

    fn create_backup(config_path: &PathBuf) {
        let backup_path_with_index = config_path.with_extension(""); // Start with the original path without extension
        let mut index = 0; // Start with 0

        // Check if the backup file already exists and append an index if it does
        loop {
            let backup_file_name = format!(
                "{}.{}.bak",
                backup_path_with_index
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                index
            );
            let backup_path = backup_path_with_index.with_file_name(backup_file_name);

            if !backup_path.exists() {
                // Copy the original config file to the backup path
                match fs::copy(config_path, &backup_path) {
                    Ok(_) => {
                        println!("Backup created at: {}", backup_path.display());
                        break; // Exit the loop after creating the backup
                    }
                    Err(e) => {
                        eprintln!("Failed to create backup of the config file: {}", e);
                        break; // Exit the loop on error
                    }
                }
            }
            index += 1; // Increment index for the next backup name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_commands() {
        let config = CargoRunner::default();

        // Test run command default
        assert_eq!(config.get_default("run".into()), Some("default"));

        // Test setting new default
        let mut config = CargoRunner::default();
        config
            .0
            .get_mut("run")
            .unwrap()
            .1
            .as_mut()
            .unwrap()
            .push(Config {
                name: "dx".to_string(),
                command_type: Some(CommandType::Shell),
                command: Some("dx".to_string()),
                sub_command: Some("serve".to_string()),
                allowed_subcommands: Some(vec![]),
                env: Some(HashMap::new()),
            });

        assert!(config.set_default(Context::Run, "dx").is_ok());
        assert_eq!(config.get_default("run".into()), Some("dx"));
    }

    #[test]
    fn test_parse_dx_config() {
        let dx_content = r#"
        [run]
        default = "dx"
        [[run.config]]
        name = "dx"
        command_type = "shell"
        command = "dx"
        sub_command = "serve"
        allowed_subcommands = ["build", "serve"]
        "#;

        let config: CargoRunner = toml::from_str(dx_content).expect("Failed to parse dx config");

        let (default, run_configs) = config.0.get("run").expect("Run config should exist");
        let run_configs = run_configs.as_ref().expect("Run config should have values");

        assert_eq!(run_configs.len(), 1);
        assert_eq!(default.as_ref().map(String::as_str), Some("dx"));

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
        let mut base_config = CargoRunner::default();

        let dx_content = r#"
        [run]
        default = "dx"
        [[run.config]]
        name = "dx"
        command_type = "shell"
        command = "dx"
        sub_command = "serve"
        allowed_subcommands = ["build", "serve"]
        "#;

        let dx_config: CargoRunner = toml::from_str(dx_content).expect("Failed to parse dx config");

        base_config.merge(dx_config);

        let (default, run_configs) = base_config.0.get("run").expect("Run config should exist");
        let run_configs = run_configs.as_ref().expect("Run config should have values");

        assert_eq!(run_configs.len(), 2);
        assert_eq!(default.as_ref().map(String::as_str), Some("dx"));

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
