use std::{error::Error, path::PathBuf};

use rx::{
    config::{CommandContext, CommandType, Config},
    config_builder::CommandDetailsBuilder,
    helper::init_config,
};

fn main() -> Result<(), Box<dyn Error>> {
    let default_config_path = PathBuf::from("rx.toml");
    init_config(default_config_path);

    let mut config: Config = Config::load(Some(PathBuf::from("rx.toml")))?;

    let command_details = CommandDetailsBuilder::new(CommandType::Cargo, "leptos")
        .params("watch")
        .build();

    let config_key = "leptos";

    let run_config = config
        .commands
        .get_or_insert_command_config(CommandContext::Run);
    run_config.update_config(config_key.to_string(), command_details);

    // If needed to remove, you can directly call it without checking for existence
    //run_config.remove_config(config_key);

    config
        .commands
        .set_default_config(CommandContext::Run, "leptos")?;

    // Save the updated configuration
    config.save(Some(PathBuf::from("rx.toml")))?;

    Ok(())
}
