use core::CargoRunner;

/// Use when the default config becomes polluted and wanna start fresh
/// This would backup the current default config
/// to a filename with format `config.$number.bak` 
/// Then replace the old config with the default config
fn main() {
    CargoRunner::reset();
}