use core::Config;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("cargo-runner-leptos.toml");
    let config = Config::load(path);
    println!("{:#?}", config);
}
