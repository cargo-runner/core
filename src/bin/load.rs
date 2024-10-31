use std::{fs::File, io::Read};

use cargo_runner::models::Config;

fn main() -> std::io::Result<()> {
    // Open the file
    let mut file = File::open("./cargo-runner.toml")?;
    
    // Read the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the contents into the Config struct
    let config: Config = toml::from_str(&contents).expect("Failed to deserialize TOML");

    // Print the loaded configuration
    println!("{:#?}", config);

    Ok(())
}
