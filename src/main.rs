use std::{error::Error, path::PathBuf};

use rx::{
    config::{CommandContext, Config},
    helper::init_config,
};

fn main() -> Result<(), Box<dyn Error>> {
    let default_config_path = PathBuf::from("rx.toml");

    init_config(default_config_path);

    let new_config = Some(PathBuf::from("rx.toml"));
    let config: Config = Config::load(new_config)?;

    println!(
        "{:#?}",
        config
            .commands
            .get_command_config(CommandContext::Run, "default")
    );

    config.save(None)?;

    Ok(())
}
