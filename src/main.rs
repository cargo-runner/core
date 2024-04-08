use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    path::PathBuf,
};

use rx::{
    builders::config::ConfigBuilder,
    errors::ConfigError,
    helpers::{
        default_config_path, ensure_config_directory_and_file, init_config, is_valid_env_var_name,
    },
    models::config::{CommandContext, CommandDetails, Config},
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

    let context = CommandContext::Run;

    let run_command_details = ConfigBuilder::new(context)
        .command(command)
        .pre_command(pre_commands)
        .env(env)
        .params(params)
        .add_validator(pre_command_validator)
        .add_validator(env_validator)
        .build()?;

    let run_config = config.commands.get_or_default_config(context);

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

#[allow(warnings)]
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
    ensure_config_directory_and_file(&config_path)?;

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
