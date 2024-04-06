use rx::config::{CommandConfig, Commands, Config}; // Adjust the path according to your actual module path

/// Test default configuration when no file is present.
#[test]
fn test_default_config() {
    let config: Config = Config::default();

    // Assuming 'default' implies standard, default, and debug for run, test, and build respectively
    assert_eq!(config.commands.run.default, "standard");
    assert_eq!(config.commands.test.default, "default");
    assert_eq!(config.commands.build.default, "debug");

    // Add more assertions here to verify the entire structure of your default Config
}
