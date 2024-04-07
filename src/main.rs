use std::{error::Error, path::Path};

use rx::{
    config::{CommandContext, Config},
    helper::init_config,
};

fn main() -> Result<(), Box<dyn Error>> {
    // note this should be from the env::args()
    let default_config_path = Path::new("rx.toml");

    init_config(default_config_path);

    // we can override Config or load different config by passing in a Path
    // If we Passed None we use the DEFAULT_CONFIG_PATH
    let config: Config = Config::load(None)?;

    println!(
        "{:#?}",
        config
            .commands
            .get_command_config(CommandContext::Run, "leptos")
    );

    Ok(())
}
