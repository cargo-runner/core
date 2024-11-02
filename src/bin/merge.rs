use cargo_runner::models::Config;
use std::{fs, path::PathBuf};
use toml;

fn main() {
    
    let mut default_config = Config::init();

    // println!("loading default config");
    // println!("{:#?}", default_config);

    // Load the second config file
    let config = Config::load(PathBuf::from("./cargo-runner-leptos.toml"));

    println!("loading data from cargo-runner-dx1.toml");
    println!("{:#?}", config);
 
    
    default_config.merge(config);

    println!("final merged config");
    println!("{:#?}", default_config);

    let toml_string = toml::to_string_pretty(&default_config)
        .expect("Failed to serialize config to TOML");

    // Write to output.toml
    fs::write("output.toml", toml_string)
        .expect("Failed to write to output.toml");

    println!("Config has been written to output.toml");

}
