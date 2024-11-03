use core::Config;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("cargo-runner-leptos.toml");
    let mut config = Config::load(path);
    {
        config.merge(Config::default());
    }

    let default = config.find("leptos");

    println!("{:#?}", default);
}
