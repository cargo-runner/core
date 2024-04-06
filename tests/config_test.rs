use rx::config::{CommandConfig, Commands, Config}; // Adjust the path according to your actual module path

/// Test default configuration when no file is present.
#[test]
fn test_default_config() {
    let config: Config = Config::default();

    assert_eq!(config.commands.run.default, "default");
    assert_eq!(config.commands.test.default, "default");
    assert_eq!(config.commands.build.default, "default");

    // command_type for all default is cargo
    let run_default_config = config.commands.run.configs.get("default").unwrap();
    assert_eq!(run_default_config.command_type, "cargo");
    assert_eq!(run_default_config.command, "run");

    let test_default_config = config.commands.test.configs.get("default").unwrap();
    assert_eq!(test_default_config.command_type, "cargo");
    assert_eq!(test_default_config.command, "test");

    let build_default_config = config.commands.build.configs.get("default").unwrap();
    assert_eq!(build_default_config.command_type, "cargo");
    assert_eq!(build_default_config.command_type, "build");
}
