use core::Config;
use std::path::PathBuf;
fn main() {
    Config::init();
    let config_path  = PathBuf::from("cargo-runner-leptos.toml");
    let config = Config::load(config_path);
    println!("{:#?}", config);
}
