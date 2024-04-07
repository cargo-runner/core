#[cfg(test)]
mod tests {
    use rx::{
        config::{CommandContext, CommandType, Config},
        config_builder::CommandDetailsBuilder,
        helper::init_config,
    };
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        // Create a temporary directory
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let config_path = temp_dir.path().join("config.toml");

        // Initialize configuration or perform other setup actions
        // For example, if `init_config` expects the config file to exist:
        fs::write(&config_path, "").expect("failed to write initial config file");
        // Or, if your init_config function creates the file if it doesn't exist:
        // init_config(config_path.clone());

        // Now load the configuration, assuming it correctly handles the provided path
        let config = Config::load(Some(config_path)).unwrap();

        // Perform your assertions here
        // For example, if you have a way to compare Config instances:
        assert_eq!(config, Config::default());

        // TempDir is automatically cleaned up when it goes out of scope
    }

    #[test]
    fn test_override_config() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let config_path = temp_dir.path().join("config.toml");

        let custom_config = r#"
[commands.run]
default = "custom"

[commands.run.configs.custom]
type = "cargo"
command = "run --package ${packageName} --bin ${binaryName}"
params = ""
allow_multiple_instances = false
working_directory = "${workspaceFolder}"
pre_command = ""

[commands.run.configs.custom.env]

    "#;

        std::fs::write(&config_path, custom_config).unwrap();
        let config = Config::load(Some(config_path)).unwrap();

        // Assert that the loaded configuration reflects the custom settings
        println!("Loaded config: {:?}", config);
        assert!(config.commands.run.is_some());
        assert_eq!(config.commands.run.unwrap().default, "custom");
    }

    #[test]
    fn test_update_and_add_config_key() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let config_path = temp_dir.path().join("config.toml");

        std::fs::write(&config_path, "").expect("failed to write initial config file");

        if !config_path.exists() {
            panic!(
                "Config file was not created at expected path: {:?}",
                config_path
            );
        }

        init_config(config_path.clone());

        let mut config = Config::load(Some(config_path.clone())).unwrap();

        let config_key = String::from("leptos");

        let new_details = CommandDetailsBuilder::new(CommandType::Cargo, "leptos watch").build();
        config
            .commands
            .get_or_insert_command_config(CommandContext::Run)
            .update_config(config_key.clone(), new_details);

        config.save(Some(config_path.clone())).unwrap();
        let reloaded_config = Config::load(Some(config_path)).unwrap();

        assert!(reloaded_config
            .commands
            .run
            .unwrap()
            .configs
            .contains_key(&config_key));
    }

    #[test]
    fn test_remove_config_key_and_default_handling() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let config_path = temp_dir.path().join("config.toml");

        std::fs::write(&config_path, "").expect("failed to write initial config file");

        if !config_path.exists() {
            panic!(
                "Config file was not created at expected path: {:?}",
                config_path
            );
        }

        init_config(config_path.clone());

        let mut config = Config::load(Some(config_path.clone())).unwrap();

        let config_key = String::from("leptos");

        let details = CommandDetailsBuilder::new(CommandType::Cargo, &config_key).build();
        config
            .commands
            .get_or_insert_command_config(CommandContext::Run)
            .update_config(config_key.clone(), details);
        config.save(Some(config_path.clone())).unwrap();

        config
            .commands
            .get_or_insert_command_config(CommandContext::Run)
            .remove_config(&config_key);
        config.save(Some(config_path.clone())).unwrap();

        let reloaded_config = Config::load(Some(config_path)).unwrap();
        assert!(!reloaded_config
            .commands
            .run
            .unwrap()
            .configs
            .contains_key(&config_key));
    }
}
