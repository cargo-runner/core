use core::CargoRunner;
use std::path::PathBuf;
fn main() {
    CargoRunner::init();
    let config_path  = PathBuf::from("cargo-runner-leptos.toml");
    let config = CargoRunner::load(config_path);
    println!("{:#?}", config);
}
