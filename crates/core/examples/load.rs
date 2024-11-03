use core::CargoRunner;
use std::path::PathBuf;

/// Use when you want to load a specific config from a given path
fn main() {
    let path = PathBuf::from("cargo-runner-leptos.toml");
    let config = CargoRunner::load(path);
    println!("{:#?}", config);
}
