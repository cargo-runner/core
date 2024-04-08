use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
};

use dirs::home_dir;
use rx::{
    config::{CommandContext, CommandDetails, CommandType, Config},
    config_builder::CommandDetailsBuilder,
    errors::ConfigError,
    helper::{init_config, is_valid_env_var_name},
    validator::Validator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config_key, command, params, env, pre_commands) = fetch_params()?;

    init_config(config_path.clone());

    let mut config: Config = Config::load(Some(config_path.clone()))?;

    let valid_pre_command_keys = config.commands.get_configs(CommandContext::Run);

    let pre_command_validator = Validator(move |details: &CommandDetails| {
        if details.pre_command.contains(config_key) {
            return Err(ConfigError::InvalidPreCommand(format!(
                "You cannot use {} as a pre_command",
                config_key
            )));
        }
        for pre_command in &details.pre_command {
            if !valid_pre_command_keys.contains(pre_command) {
                return Err(ConfigError::InvalidPreCommand(format!(
                    "Pre-command must be any of the following: [{}]",
                    valid_pre_command_keys.join(",")
                )));
            }
        }
        Ok(())
    });

    let env_validator = Validator(move |details| {
        for key in details.env.keys() {
            if !is_valid_env_var_name(key) {
                return Err(ConfigError::InvalidEnvFormat);
            }
        }
        Ok(())
    });

    let run_command_details = CommandDetailsBuilder::new(CommandType::Cargo, command)
        .pre_command(pre_commands)
        .env(env)
        .params(params)
        .add_validator(pre_command_validator)
        .add_validator(env_validator)
        .build()?;

    let run_config = config.commands.get_or_default_config(CommandContext::Run);

    run_config.update_config(config_key, run_command_details);

    config
        .commands
        .set_default_config(CommandContext::Run, config_key)?;

    config.save(Some(config_path))?;

    Ok(())
}
fn get_config_path() -> PathBuf {
    std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_config_path)
}
fn default_config_path() -> PathBuf {
    home_dir()
        .expect("Could not find home directory")
        .join(".config/cargo_runner/config.toml")
}
fn ensure_config_directory_and_file(path: &PathBuf) {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create configuration directory");
        }
        create_default_config_file(path);
    }
}
fn create_default_config_file(path: &PathBuf) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("Failed to create default configuration file");

    let default_config = Config::default();
    let toml = toml::to_string(&default_config).expect("Failed to serialize default configuration");
    writeln!(file, "{}", toml).expect("Failed to write default configuration");
}

fn fetch_params<'a>() -> Result<
    (
        PathBuf,
        &'a str,
        &'a str,
        &'a str,
        HashMap<String, String>,
        BTreeSet<String>,
    ),
    Box<dyn Error>,
> {
    let config_path = get_config_path();
    ensure_config_directory_and_file(&config_path);

    let pre_commands: BTreeSet<String> = ["default"].into_iter().map(String::from).collect();

    let config_key = "leptos";
    let params = "watch";

    let command = "leptos";
    let env: HashMap<String, String> = [
        ("APP_NAME", "Cargo Runner"),
        ("MY_CUSTOM_VAR_1", "TRUE"),
        ("COPY_TRAIT", "FALSE"),
    ]
    .iter()
    .map(|(k, v)| (String::from(*k), String::from(*v)))
    .collect();
    Ok((config_path, config_key, command, params, env, pre_commands))
}
