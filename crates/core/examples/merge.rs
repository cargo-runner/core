use core::{CargoRunner, Context};
use std::path::PathBuf;

/// Use when you want to merge a specific config and override it with another config
fn main() {
    let mut config = CargoRunner::default();

    let path = PathBuf::from("cargo-runner-leptos.toml");

    let leptos_config = CargoRunner::load(path);

    config.merge(leptos_config);

    let default = config.get_default(Context::Run);

    println!(
        "run default command config is set to: {:#?}",
        default.unwrap_or_default()
    );

    println!("{:#?}", config);
}
