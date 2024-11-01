use cargo_runner::models::Config;
use std::fs;
use toml;

fn main() {
    

    // Load the second config file
    let data = fs::read_to_string("cargo-runner-dx1.toml")
        .expect("Failed to read cargo-runner-dx1.toml");
    let config: Config = toml::from_str(&data)
        .expect("Failed to parse cargo-runner-dx1.toml");

    println!("loading data from cargo-runner-dx1.toml");
    println!("{:#?}", config);
 


    // Create a mutable base config and merge the others into it
    let mut final_config = Config::default_config();

    println!("loading default config");
    println!("{:#?}", final_config);
    final_config.merge(config);
    println!("final merged config");
    println!("{:#?}", final_config);

    let toml_string = toml::to_string_pretty(&final_config)
        .expect("Failed to serialize config to TOML");

    // Write to output.toml
    fs::write("output.toml", toml_string)
        .expect("Failed to write to output.toml");

    println!("Config has been written to output.toml");


    
}
