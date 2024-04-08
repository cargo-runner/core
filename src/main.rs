use std::{collections::BTreeSet, path::PathBuf};

use rx::{
    config::{CommandContext, CommandDetails, CommandType, Config},
    config_builder::CommandDetailsBuilder,
    errors::ConfigError,
    helper::init_config,
    validator::Validator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_str = "rx.toml";

    let default_config_path = PathBuf::from(path_str);

    init_config(default_config_path.clone());

    let mut config: Config = Config::load(Some(default_config_path.clone()))?;

    let pre_commands: BTreeSet<String> = ["leptos"].into_iter().map(String::from).collect();

    let config_key = "leptos";

    let valid_pre_command_keys = config.commands.get_configs(CommandContext::Run);

    let pre_command_validator = Validator(move |details: &CommandDetails| {
        if details.pre_command.contains(config_key) {
            return Err(ConfigError::InvalidPreCommand(format!(
                "You cannot use {} as a pre_command",
                config_key
            )));
        }
        for pre_command in &details.pre_command {
            if !valid_pre_command_keys.contains(pre_command) {
                return Err(ConfigError::InvalidPreCommand(format!(
                    "Pre-command must be any of the following: [{}]",
                    valid_pre_command_keys.join(",")
                )));
            }
        }

        Ok(())
    });

    let command = "leptos";

    let run_command_details = CommandDetailsBuilder::new(CommandType::Cargo, command)
        .command("leptos")
        .pre_command(pre_commands)
        .params("watch")
        .add_validator(pre_command_validator)
        .build()?;

    let run_config = config
        .commands
        .get_or_insert_command_config(CommandContext::Run);
    run_config.update_config(config_key, run_command_details);

    config.save(Some(default_config_path))?;

    Ok(())
}
