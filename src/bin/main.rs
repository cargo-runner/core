use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    path::PathBuf,
};

use cargo_runner::{
    errors::ConfigError,
    helpers::{
        default_config_path, ensure_config_directory_and_file, init_config, is_valid_env_var_name,
    },
    models::{ CommandDetails, CommandType, Config,CargoContext},
    validator::Validator,
    CargoConfigBuilder
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config_key, sub_command, sub_action, params, env, allowed_subcommands) =
        fetch_params()?;

    init_config(config_path.clone());

    let mut config: Config = Config::load(Some(config_path.clone()))?;

    let configs_keys = config.context.get_configs(CargoContext::Run);
    eprintln!("configs_keys: {:?}", configs_keys);

    let sub_command_validator = Validator(move |details: &CommandDetails| {
        if !details.allowed_subcommands.contains(sub_command) {
            return Err(ConfigError::InvalidSubCommand(format!(
                "You cannot use {} as a sub_command",
                sub_command
            )));
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

    let command_type = CommandType::Command("dx".to_string());
    let context = CargoContext::Run;

    let run_command_details = CargoConfigBuilder::new(command_type.clone(), context)
        .command(String::from(command_type).as_str())
        .sub_command(sub_command)
        .sub_action(sub_action)
        .allowed_subcommands(allowed_subcommands)
        .env(env)
        .params(params)
        .add_validator(sub_command_validator)
        .add_validator(env_validator)
        .build()?;

    let run_config = config.context.get_or_default_config(context);

    run_config.update_config(config_key, run_command_details);

    config
        .context
        .set_default_config(CargoContext::Run, config_key)?;

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
        &'a str,
        HashMap<String, String>,
        BTreeSet<String>,
    ),
    Box<dyn Error>,
> {
    let config_path = get_config_path();
    ensure_config_directory_and_file(&config_path)?;

    let allowed_subcommands: BTreeSet<String> = [
        "build",
        "translate",
        "serve",
        "new",
        "init",
        "clean",
        "bundle",
        "fmt",
        "check",
        "config",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let config_key = "dx";
    let sub_command = "build";
    let sub_action = "";
    let params = "";

    let env: HashMap<String, String> = [("WARNINGS", "TRUE")]
        .iter()
        .map(|(k, v)| (String::from(*k), String::from(*v)))
        .collect();
    Ok((
        config_path,
        config_key,
        sub_command,
        sub_action,
        params,
        env,
        allowed_subcommands,
    ))
}
