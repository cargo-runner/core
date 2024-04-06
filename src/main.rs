use std::error::Error;

use rx::{config::Config, global::APP_CONFIG, helper::read_file};

fn main() -> Result<(), Box<dyn Error>> {
    // Assume you have this filename from somewhere
    let filename = "rx.toml";
    read_file(filename)?;

    // Lock the Mutex and get a reference to the String, don't try to move it
    let file_content = APP_CONFIG.lock().unwrap();

    let config: Config = toml::from_str(&file_content)?;

    println!("{:#?}", config);

    Ok(())
}
