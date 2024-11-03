use core::{Config, Context};
use std::path::PathBuf;

fn main() {
    let mut config = Config::default();

    let path = PathBuf::from("cargo-runner-leptos.toml");

    let leptos_config = Config::load(path);

    config.merge(leptos_config);

    let default = config.get_default(Context::Run);

    println!(
        "run default command config is set to: {:#?}",
        default.unwrap_or_default()
    );

    println!("{:#?}", config);
}
