# Cargo Runner

## Changelog

## April 08, 2024

- [X] `Commands` Have Ability to Add Zero or More `CommandConfig`
- [X] `Commands` can `get_configs(CommandContext)`
- [X] `ConfigBuilder` on `new` must require `command` and `command_type`
- [X] `ConfigBuilder` can chain different builder methods
  - command
  - env
  - pre_command
  - params
  - working_directory
  - allow_multiple_instances
  - command_type
- [X] `ConfigBuilder` can `add_validator` using  `Validator()` closure
- [X] `ConfigBuilder` uses `build` to turn it into `CommandDetails`
- [X] `ConfigBuilder` can use `get_or_default_config(CommandContext)`
- [X] `ConfigBuilder` instance can `update_config(config_key, run_command_details)`
- [X] `Commands` can `set_default_config(CommandContext, config_key)`
- [X] Ensure Config Deserialization for empty field and file by setting defaults with serde macro
- [X] `Config` instance can `save` after modifying `Config`
- [X] `Config` can `load` config file on init of the app
- [X] `ConfigError` is used when `Error` on `Config` happens
- [X] Create Default Config file on `$HOME/.config/cargo_runner/config.toml`
