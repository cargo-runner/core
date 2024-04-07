use std::{error::Error, path::Path};

use rx::{
    config::{CommandContext, Config},
    global::CONFIGURATION_FILE,
    helper::{init_default_config, read_file},
};

fn main() -> Result<(), Box<dyn Error>> {
    init_default_config();
    // Assume you have this filename from somewhere
    let filename = Path::new("rx.toml");
    read_file(filename)?;

    // Lock the Mutex and get a reference to the String, don't try to move it
    let file_content = CONFIGURATION_FILE.lock().unwrap();

    let config: Config = toml::from_str(&file_content).unwrap_or(Config::default());

    println!(
        "{:#?}",
        config
            .commands
            .get_command_config(CommandContext::Run, "leptos")
    );

    Ok(())
}
