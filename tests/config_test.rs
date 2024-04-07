#[cfg(test)]
mod tests {
    use rx::{
        config::{CommandContext, CommandType, Config},
        config_builder::CommandDetailsBuilder,
        helper::init_config,
    };
    use std::{fs, path::PathBuf};
    use tempfile::TempDir;

    /// Sets up the test environment, including creating a temporary config file.
    /// Optionally accepts custom configuration content as a string.
    ///
    /// # Arguments
    ///
    /// * `custom_config` - Optional custom configuration content to write to the config file.
    ///
    /// # Returns
    ///
    /// A tuple containing the loaded [Config] instance and the [PathBuf] to the temporary config file.
    ///
    /// The [TempDir] must be drop manually at the last line of the function so the file path wont be
    /// destroyed and thus avoiding error FileNotFound
    fn setup(custom_config: Option<&str>) -> (Config, PathBuf, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create a temporary directory");
        let config_path = temp_dir.path().join("config.toml");

        // Determine the configuration content to write: custom or empty
        let config_content = custom_config.unwrap_or("");

        // Write the determined configuration content to the file
        fs::write(&config_path, config_content).expect("Failed to write to the config file");

        // Assuming `init_config` does necessary initializations based on the config file
        init_config(config_path.clone());

        // Load the configuration from the newly created temp file
        let config = Config::load(Some(config_path.clone())).unwrap();

        // Return the loaded configuration, the path to the config file, and the TempDir instance
        (config, config_path, temp_dir)
    }

    #[test]
    fn test_default_config() {
        let (config, _, _) = setup(None);

        // For each command, check existence and default details
        let run_config = config.commands.run.expect("run config should have default");
        assert_eq!(run_config.default, "default");
        let run_details = run_config
            .configs
            .get("default")
            .expect("default run config should exist");
        assert_eq!(
            run_details.command.as_deref(),
            Some("run --package ${packageName} --bin ${binaryName}")
        );
        assert_eq!(run_details.command_type, CommandType::Cargo);

        let test_config = config
            .commands
            .test
            .expect("test config should have default");
        assert_eq!(test_config.default, "default");
        let test_details = test_config
            .configs
            .get("default")
            .expect("default test config should exist");
        assert_eq!(test_details.command.as_deref(), Some("test"));
        assert_eq!(test_details.command_type, CommandType::Cargo);

        let build_config = config
            .commands
            .build
            .expect("build config should have default");
        assert_eq!(build_config.default, "default");
        let build_details = build_config
            .configs
            .get("default")
            .expect("default build config should exist");
        assert_eq!(build_details.command.as_deref(), Some("build"));
        assert_eq!(build_details.command_type, CommandType::Cargo);

        let bench_config = config
            .commands
            .bench
            .expect("bench config should have default");
        assert_eq!(bench_config.default, "default");

        let bench_details = bench_config
            .configs
            .get("default")
            .expect("default bench config should exist");

        assert_eq!(bench_details.command.as_deref(), Some("bench"));
        assert_eq!(bench_details.command_type, CommandType::Cargo);

        assert!(
            config.commands.script.is_none(),
            "script config is none by default"
        );
    }

    #[test]
    fn test_override_config() {
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
env = {}
"#;

        let (config, _config_path, _temp_dir) = setup(Some(custom_config));

        // Assert that the 'run' configuration exists and has 'custom' as the default
        let run_config = config.commands.run.expect("Run configuration should exist");
        assert_eq!(
            run_config.default, "custom",
            "Default should be set to 'custom'"
        );

        // Verify the existence and contents of the 'custom' config
        let custom_config_details = run_config
            .configs
            .get("custom")
            .expect("Custom config should exist under run");
        assert_eq!(
            custom_config_details.command.as_deref(),
            Some("run --package ${packageName} --bin ${binaryName}"),
            "Command does not match"
        );
        assert_eq!(
            custom_config_details.params.as_deref(),
            Some(""),
            "Params do not match"
        );
        assert_eq!(
            custom_config_details.allow_multiple_instances,
            Some(false),
            "Allow multiple instances flag does not match"
        );
        assert_eq!(
            custom_config_details.working_directory.as_deref(),
            Some("${workspaceFolder}"),
            "Working directory does not match"
        );
        assert!(
            custom_config_details.env.is_some()
                && custom_config_details.env.as_ref().unwrap().is_empty(),
            "Env should be empty"
        );
    }

    #[test]
    fn test_update_and_add_config_key() {
        let (mut config, config_path, _temp_dir) = setup(None); // Use _temp_dir to hold onto the TempDir instance

        let config_key = String::from("leptos");

        let new_details = CommandDetailsBuilder::new(CommandType::Cargo, "leptos watch").build();

        // Update the configuration for 'Run' context with new details
        let run_config = config
            .commands
            .get_or_insert_command_config(CommandContext::Run);
        run_config.update_config(config_key.clone(), new_details);

        // Save the updated configuration
        assert!(
            config.save(Some(config_path.clone())).is_ok(),
            "Failed to save updated configuration"
        );

        // Reload the configuration to verify updates
        let reloaded_config =
            Config::load(Some(config_path.clone())).expect("Failed to reload configuration");

        // Assert that the updated details are present
        assert!(
            reloaded_config.commands.run.is_some(),
            "Run configuration should exist"
        );
        let reloaded_run_config = reloaded_config.commands.run.unwrap();
        assert!(
            reloaded_run_config.configs.contains_key(&config_key),
            "Config key 'leptos' should be present in the run configs"
        );

        let updated_details = reloaded_run_config
            .configs
            .get(&config_key)
            .expect("Config key 'leptos' was not found after update");
        assert_eq!(
            updated_details.command.as_deref(),
            Some("leptos watch"),
            "Command for 'leptos' config key did not match expected value"
        );
    }

    #[test]
    fn test_remove_config_key_and_default_handling() {
        let (mut config, config_path, _temp_dir) = setup(None);

        let config_key = "leptos";
        let details = CommandDetailsBuilder::new(CommandType::Cargo, "cargo watch").build();

        // add leptos to configs
        {
            let run_config = config
                .commands
                .get_or_insert_command_config(CommandContext::Run);
            run_config.update_config(config_key.to_string(), details);
        }

        // Initially set 'leptos' as the default
        // note this would error out if leptos is not created yet
        {
            assert!(
                config
                    .commands
                    .set_default_config(CommandContext::Run, config_key)
                    .is_ok(),
                "Setting default config failed"
            );
        }

        // Now, remove the 'leptos' config key
        {
            let run_config = config
                .commands
                .get_or_insert_command_config(CommandContext::Run);

            run_config.remove_config(config_key);
        }

        // Save the updated configuration
        assert!(
            config.save(Some(config_path.clone())).is_ok(),
            "Failed to save configuration after removal"
        );

        // Reload the configuration to verify that the removal persists
        let reloaded_config =
            Config::load(Some(config_path.clone())).expect("Failed to reload configuration");

        // Verify 'leptos' has been removed
        assert!(
            reloaded_config.commands.run.is_some(),
            "Run configuration should exist"
        );

        let reloaded_run_config = reloaded_config.commands.run.unwrap();
        assert!(
            !reloaded_run_config.configs.contains_key(config_key),
            "Config key 'leptos' should have been removed"
        );

        // Optionally, test how the removal affects the default config
        // This will depend on your implementation of `set_default_config` and `remove_config`
        // For example, you might check if the default has been reverted to another key
        assert_ne!(
            reloaded_run_config.default, config_key,
            "Default config key should no longer be 'leptos'"
        );
    }
}
