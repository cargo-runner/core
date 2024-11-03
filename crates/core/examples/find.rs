use core::{CargoRunner, Context};
use std::path::PathBuf;

/// Use when you want to find a specific config for a given context
fn main() {
    let mut config = CargoRunner::default();
    let path = PathBuf::from("example-leptos.toml");
    let  leptos = CargoRunner::load(path);
    {
        config.merge(leptos);
    }

    let default = config.find(Context::Run,"leptos");

    println!("{:#?}", default);
}
