use std::{collections::BTreeSet, path::PathBuf};

use rx::{
    config::{CommandContext, CommandDetails, CommandType, Config},
    config_builder::CommandDetailsBuilder,
    errors::ConfigError,
    helper::init_config,
    types::CommandDetailsValidation,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_str = "rx.toml";
    let default_config_path = PathBuf::from(path_str);

    init_config(default_config_path.clone());

    let mut config: Config = Config::load(Some(default_config_path.clone()))?;

    let pre_commands: BTreeSet<String> = ["leptos"].into_iter().map(String::from).collect();

    let valid_pre_command_keys = config.commands.get_configs(CommandContext::Run);
    let validate_pre_commands: CommandDetailsValidation =
        Box::new(move |details: &CommandDetails| {
            for pre_command in &details.pre_command {
                if !valid_pre_command_keys.contains(pre_command) {
                    return Err(ConfigError::InvalidPreCommand(format!(
                        "Invalid pre-command: '{}'. Must be one of: [{}]",
                        pre_command,
                        valid_pre_command_keys.join(",")
                    )));
                }
            }
            Ok(())
        });

    let run_command_details = CommandDetailsBuilder::new(CommandType::Cargo, "leptos")
        .pre_command(pre_commands)
        .params("watch")
        .build(vec![validate_pre_commands])?;

    let config_key = "leptos";

    let run_config = config
        .commands
        .get_or_insert_command_config(CommandContext::Run);
    run_config.update_config(config_key, run_command_details);

    config.save(Some(default_config_path))?;

    Ok(())
}
