use core::CargoRunner;

/// Use when you want to initialize a new config at `~/.cargo-runner/config.toml`
fn main() {
    let config = CargoRunner::init();
    println!("{:#?}", config);
}
