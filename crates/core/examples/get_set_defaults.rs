use core::{CargoRunner, Context};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("cargo-runner-leptos.toml");

    let mut config = CargoRunner::load(path);

    config.merge(CargoRunner::default());

    let default = config.get_default(Context::Run);

    println!(
        "previous default for run context: {:#?}",
        default.unwrap_or_default()
    );

    config.set_default(Context::Run, "leptos").unwrap();

    let default = config.get_default(Context::Run);

    println!(
        "latest default for run context: {:#?}",
        default.unwrap_or_default()
    );
}
