# Rx (Rust Command Executor)

A Cli tools use to build Cargo / Command Runner Config on your IDE of choice

This package focus on Parsing the Config file, Providing default configuration , and Updating Configs

Note: You can have multiple Configs on your Project

The Default Path where the Config File would be located is on on your $WorkspaceFolder

We can load our default config

```rust
let default_config_path = PathBuf::from("rx.toml");

    init_config(default_config_path);
    // override default_config_path with specified path
    let new_config = Some(PathBuf::from("rx.toml"));
    let config: Config = Config::load(new_config)?;

    // or we can load and use the DEFAULT_CONFIG_PATH
    let config: Config = Config::load(None)?;
```


<details>
  <summary>Example of rx.toml config file</summary>
  </br>
```toml
[commands]

## Run Configuration
[commands.run]
default = "dioxus"

[commands.run.configs.default]
type = "cargo"
command = "run --package ${packageName} --bin ${binaryName}"
params = ""
env = {}
allow_multiple_instances = false
working_directory = "${workspaceFolder}"
pre_command = ""

[commands.run.configs.watch]
type = "cargo"
command = "watch -x run"
params = ""
env = {}
allow_multiple_instances = true
working_directory = "${workspaceFolder}"
pre_command = ""

[commands.run.configs.leptos]
type = "shell"
command = "cargo leptos watch"
params = ""
env = {}
allow_multiple_instances = true
working_directory = "${workspaceFolder}"
pre_command = ""

[commands.run.configs.dioxus]
type = "shell"
command = "dx serve --hot-reload"
params = ""
env = {}
allow_multiple_instances = true
working_directory = "${workspaceFolder}"
pre_command = ""

## Test Configuration
[commands.test]
default = "default"

[commands.test.configs.default]
type = "cargo"
command = "test --package ${packageName}"
params = ""
env = {}
allow_multiple_instances = true
working_directory = "${workspaceFolder}"
pre_command = ""

## Build Configuration
[commands.build]
default = "default"

[commands.build.configs.default]
type = "cargo"
command = "build --package ${packageName}"
params = ""
env = {}
allow_multiple_instances = false
working_directory = "${workspaceFolder}"
pre_command = ""

[commands.script]
default = "default"

[commands.script.configs.default]
type = "shell"
command = "bunx tailwindcss -i ./input.css -o ./public/tailwind.css --watch"
params = ""
env = {}
allow_multiple_instances = false
working_directory = "${workspaceFolder}"
pre_command = ""
```
</br>
</details>


Note: `CommandConfig` can either be any of the `CommandContext`

If you dont provide any `CommandConfig` for specific `CommandContext`

It would use `DEFAULT_${CommandContext}_CONFIG` GLOBALS

and that is set using this method on `init_default_config()`


```sh
CommandConfig::with_context("run")
```

To Access specific `CommandConfig` key
use

```sh
config
      .commands
      .get_command_config(CommandContext::Run, "leptos")
```

where the second param is the name of your config_name if the config_name dont exist it just return None
