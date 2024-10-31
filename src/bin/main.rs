use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use cargo_runner::models::{CommandConfig, CommandType, Config};

fn main() -> std::io::Result<()> {

    let run_config = vec![
        CommandConfig {
            name: "default".to_string(),
            command_type: CommandType::Cargo,
            command: "cargo".to_string(),
            sub_command: "run".to_string(),
            allowed_subcommands: vec![],
            env: HashMap::new(),
        },
        CommandConfig {
            name: "leptos".to_string(),
            command_type: CommandType::SubCommand,
            command: "cargo".to_string(),
            sub_command: "leptos".to_string(),
            allowed_subcommands: vec![],
            env: HashMap::new()
        },
        CommandConfig {
            name: "dx".to_string(),
            command_type: CommandType::Shell("dx".into()),
            command: "dx".to_string(),
            sub_command: "build".to_string(),
            allowed_subcommands: vec![
                "build".to_string(), "bundle".to_string(), "check".to_string(), "clean".to_string(), "config".to_string(), "fmt".to_string(), "init".to_string(), "new".to_string(), "serve".to_string(), "translate".to_string(),
            ],
            env: HashMap::new(),
        },
    ];

    let mut test_env = HashMap::new();
    test_env.insert("RUSTFLAGS".to_string(), "-Awarnings".to_string());

    let test_config = vec![CommandConfig {
        name: "default".to_string(),
        command_type: CommandType::Cargo,
        command: "cargo".to_string(),
        sub_command: "test".to_string(),
        allowed_subcommands: vec![],
        env: test_env,
    }];

    let build_config = vec![CommandConfig {
        name: "default".to_string(),
        command_type: CommandType::Cargo,
        command: "cargo".to_string(),
        sub_command: "build".to_string(),
        allowed_subcommands: vec![],
        env: HashMap::new(),
    }];

    let bench_config = vec![CommandConfig {
        name: "default".to_string(),
        command_type: CommandType::Cargo,
        command: "cargo".to_string(),
        sub_command: "bench".to_string(),
        allowed_subcommands: vec![],
        env: HashMap::new(),
    }];

    let config = Config {
        run: run_config,
        test: test_config,
        build: build_config,
        bench: bench_config,
    };

    let toml_string = toml::to_string(&config).expect("Failed to serialize to TOML");

    let mut file = File::create("cargo-runner.toml")?;
    file.write_all(toml_string.as_bytes())?;

    println!("Configuration written to cargo-runner.toml");
    Ok(())
}
