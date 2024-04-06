use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rx::{
    config::Config,
    helper::{read_file, GlobalString},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file_str: GlobalString = Arc::new(Mutex::new(String::new()));

    // Assume you have this filename from somewhere
    let filename = "rx.toml";
    read_file(filename, &file_str)?;

    // Lock the Mutex and get a reference to the String, don't try to move it
    let file_content = file_str.lock().unwrap();

    let config: Config = toml::from_str(&file_content)?;

    println!("{:#?}", config);

    Ok(())
}
