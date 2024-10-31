use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use dirs::home_dir;

use crate::{
    global::{
        CONFIGURATION_FILE_CONTENT, DEFAULT_BENCH_CONFIG, DEFAULT_BUILD_CONFIG,
        DEFAULT_CONFIG_PATH, DEFAULT_RUN_CONFIG,  DEFAULT_TEST_CONFIG,
    },
    models::{CommandConfig,  Cargo, CargoContext}
};

fn append_new_line(data: &str) {
    CONFIGURATION_FILE_CONTENT
        .lock()
        .unwrap()
        .push_str(&(data.to_string() + "\n"));
}

pub fn read_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (number, line) in reader.lines().enumerate() {
        match line {
            Ok(text) => {
                append_new_line(&text);
            }
            Err(_) => println!("Error reading line {}", number + 1),
        }
    }
    Ok(())
}

pub fn write_to_config_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in content.lines() {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn init_config(default_path: PathBuf) {
    let _ = DEFAULT_CONFIG_PATH.set(default_path);
    let _ = DEFAULT_RUN_CONFIG.set(CommandConfig::with_context(CargoContext::Run));
    let _ = DEFAULT_TEST_CONFIG.set(CommandConfig::with_context(CargoContext::Test));
    let _ = DEFAULT_BUILD_CONFIG.set(CommandConfig::with_context(CargoContext::Build));
    let _ = DEFAULT_BENCH_CONFIG.set(CommandConfig::with_context(CargoContext::Bench));
}

pub fn is_all_caps(s: &str) -> bool {
    s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

pub fn is_valid_env_var_name(name: &str) -> bool {
    let mut chars = name.chars();

    // Check if the first character is an uppercase letter
    match chars.next() {
        Some(first_char) => first_char.is_ascii_uppercase(),
        None => return false, // The name is empty, so it's invalid
    };

    // Check the rest of the characters
    chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

pub fn default_config_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(".config/cargo_runner/config.toml")
}
pub fn ensure_config_directory_and_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create configuration directory");
        }
        create_default_config_file(path)?;
    }
    Ok(())
}
pub fn create_default_config_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let default_config = Cargo::default();
    let toml = toml::to_string(&default_config)?;
    write_to_config_file(path, &toml)?;
    Ok(())
}
