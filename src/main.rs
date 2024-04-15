use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    path::PathBuf,
};

use rx::{
    builders::config::ConfigBuilder,
    helpers::{default_config_path, ensure_config_directory_and_file, init_config},
    models::config::{CommandContext, Config},
    validator::{command_type_validator, command_validator, pre_command_validator},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, config_key, command, params, env, pre_commands) = fetch_params()?;

    init_config(config_path.clone());

    let mut config: Config = Config::load(Some(config_path.clone()))?;

    let context = CommandContext::Run;

 

    let run_command_details = ConfigBuilder::new(context)
        .config_key(config_key)
        .command(command)
        .pre_command(pre_commands)
        .env(env)
        .params(params)
        .add_validator(pre_command_validator())
        .add_validator(command_validator())
        .add_validator(command_type_validator())
        .build(&config)?;

    let run_config = config.commands.get_or_default_config(context);

    run_config.update_config(config_key, run_command_details);

    config
        .commands
        .set_default_config(context, config_key)?;

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
