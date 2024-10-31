#[cfg(test)]
mod tests {
    use std::{fs, iter::repeat_with, path::PathBuf};

    use cargo_runner::{
        helpers::init_config,
        models::{ ContextType, CommandType, Config},
        CargoConfigBuilder
    };
    use rand::Rng;
    use tempfile::Builder;
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
        let random_prefix: String = repeat_with(|| rand::thread_rng().gen_range(0..36))
            .map(|n| {
                if n < 10 {
                    (b'0' + n) as char
                } else {
                    (b'a' + n - 10) as char
                }
            })
            .take(8)
            .collect();

        // Use a new TempDir instantiation directly, ensuring it's unique
        let temp_dir = Builder::new()
            .prefix(&random_prefix)
            .tempdir()
            .expect("Failed to create a temporary directory");
        let config_path = temp_dir.path().join("config.toml");

        let config_content = custom_config.unwrap_or("");
        fs::write(&config_path, config_content).expect("Failed to write to the config file");

        // Initialize and load the configuration
        init_config(config_path.clone());
        let config = Config::load(Some(config_path.clone())).expect("Loading Config Failed");

        // Return the loaded configuration, config file path, and temp directory
        (config, config_path, temp_dir)
    }

    #[test]
    fn test_default_config() {
        let (config, _, temp_dir) = setup(None);
        eprintln!("{:?}", temp_dir);

        // For each command, check existence and default details
        let run_config = config.context.run.expect("run config should have default");
        assert_eq!(run_config.default, "default");
        let run_details = run_config
            .configs
            .get("default")
            .expect("default run config should exist");
        assert_eq!(run_details.sub_command, "run");
        assert_eq!(run_details.command_type, CommandType::Cargo);

        let test_config = config
            .context
            .test
            .expect("test config should have default");
        assert_eq!(test_config.default, "default");
        let test_details = test_config
            .configs
            .get("default")
            .expect("default test config should exist");
        assert_eq!(test_details.sub_command, "test");
        assert_eq!(test_details.command_type, CommandType::Cargo);

        let build_config = config
            .context
            .build
            .expect("build config should have default");
        assert_eq!(build_config.default, "default");
        let build_details = build_config
            .configs
            .get("default")
            .expect("default build config should exist");
        assert_eq!(build_details.sub_command, "build");
        assert_eq!(build_details.command_type, CommandType::Cargo);

        let bench_config = config
            .context
            .bench
            .expect("bench config should have default");
        assert_eq!(bench_config.default, "default");

        let bench_details = bench_config
            .configs
            .get("default")
            .expect("default bench config should exist");

        assert_eq!(bench_details.sub_command, "bench");
        assert_eq!(bench_details.command_type, CommandType::Cargo);
    }

    #[test]
    fn test_config_load() {
        let custom_config = r#"
[context.run]
default = "leptos"

[context.run.configs.leptos]
type = "cargo"  # Changed from "sub-command" to "cargo"
command = "cargo"
sub_command = "leptos"
sub_action = "watch"
params = ""
allowed_subcommands = []

[context.run.configs.leptos.env]

[context.run.configs.default]
type = "cargo"
command = "cargo"
sub_command = "run"
sub_action = ""
params = ""
allowed_subcommands = []

[context.run.configs.default.env]

[context.test]
default = "default"

[context.test.configs.default]
type = "cargo"
command = "cargo"
sub_command = "test"
sub_action = ""
params = ""
allowed_subcommands = []

[context.test.configs.default.env]
"#;

        let (config, _config_path, temp_dir) = setup(Some(custom_config));

        eprintln!("{:?}", temp_dir);

        assert_eq!(
            config
                .context
                .run
                .expect("Run configuration should exist")
                .default,
            "leptos"
        );
        // Additional assertions...
    }

    #[test]
    fn test_update_and_add_config_key() {
        let (mut config, config_path, temp_dir) = setup(None);

        eprintln!("{:?}", temp_dir);

        let config_key = "leptos";

        let context = ContextType::Run;

        let command_type = CommandType::SubCommand;

        let sub_command = "leptos";

        let sub_action = "watch";

        let new_details = CargoConfigBuilder::new(command_type.clone(), context)
            .command_type(CommandType::Cargo)
            .command(String::from(command_type).as_str())
            .sub_command(sub_command)
            .sub_action(sub_action)
            .build()
            .unwrap();

        // Update the configuration for 'Run' context with new details
        {
            let run_config = config.context.get_or_default_config(context);
            run_config.update_config(config_key, new_details);
        }

        assert!(
            config.save(Some(config_path.clone())).is_ok(),
            "Failed to save updated configuration"
        );

        let reloaded_config =
            Config::load(Some(config_path.clone())).expect("Failed to reload configuration");

        assert!(
            reloaded_config.context.run.is_some(),
            "Run configuration should exist"
        );
        assert!(
            reloaded_config
                .context
                .run
                .unwrap()
                .configs
                .contains_key(config_key),
            "Config key 'leptos' should be present in the run configs"
        );
    }

    #[test]
    fn test_remove_config_key_and_default_handling() {
        let (mut config, config_path, temp_dir) = setup(None);

        eprintln!("{:?}", temp_dir);

        let config_key = "leptos";

        let context = ContextType::Run;

        let command_type = CommandType::SubCommand;

        let sub_command = "leptos";

        let sub_action = "watch";

        let details = CargoConfigBuilder::new(command_type.clone(), context)
            .command_type(command_type.clone())
            .command(String::from(command_type).as_str())
            .sub_command(sub_command)
            .sub_action(sub_action)
            .build()
            .unwrap();

        // add leptos to configs
        {
            let run_config = config.context.get_or_default_config(context);
            run_config.update_config(config_key, details);
        }

        // Initially set 'leptos' as the default
        // note this would error out if leptos is not created yet
        {
            assert!(
                config
                    .context
                    .set_default_config(context, config_key)
                    .is_ok(),
                "Setting default config failed"
            );
        }

        // Now, remove the 'leptos' config key
        {
            let run_config = config.context.get_or_default_config(context);

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
            reloaded_config.context.run.is_some(),
            "Run configuration should exist"
        );

        let reloaded_run_config = reloaded_config.context.run.unwrap();
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
