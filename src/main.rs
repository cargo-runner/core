use std::{error::Error, path::Path};

use rx::{
    config::{CommandContext, Config},
    helper::init_default_config,
};

fn main() -> Result<(), Box<dyn Error>> {
    init_default_config();
    let filename = Path::new("rx.toml");

    let config: Config = Config::load(filename)?;

    println!(
        "{:#?}",
        config
            .commands
            .get_command_config(CommandContext::Run, "leptos")
    );

    Ok(())
}
