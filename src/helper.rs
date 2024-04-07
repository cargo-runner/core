use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{
    config::CommandConfig,
    global::{
        CONFIGURATION_FILE_CONTENT, DEFAULT_BENCH_CONFIG, DEFAULT_BUILD_CONFIG,
        DEFAULT_CONFIG_PATH, DEFAULT_RUN_CONFIG, DEFAULT_SCRIPT_CONFIG, DEFAULT_TEST_CONFIG,
    },
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
    let _ = DEFAULT_RUN_CONFIG.set(CommandConfig::with_context("run"));
    let _ = DEFAULT_TEST_CONFIG.set(CommandConfig::with_context("test"));
    let _ = DEFAULT_BUILD_CONFIG.set(CommandConfig::with_context("build"));
    let _ = DEFAULT_BENCH_CONFIG.set(CommandConfig::with_context("bench"));
    let _ = DEFAULT_SCRIPT_CONFIG.set(CommandConfig::with_context("script"));
}
