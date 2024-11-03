# Cargo Runner

A Vscode plugin for 10X Rust Developer Tool to Run, Build, or Test without Mental Overhead.

## Techstacks

- TypeScript
- Rust
- WASM
- WIT (WebAssembly Interface Types)


## Cargo Runner Core

### Features

#### a. Config Builder CLI

> Note: This can be instaled optionally with `cargo install cargo-runner-cli` or `cargo install --git https://github.com/cargo-runner/cli` 

VSCode plugin will have this functionality using `Commmand Palette`

<details>
<summary>Generate Default config on <em>~/.cargo-runner/config.toml</em></summary>

```sh
cf init
```

</details>

<details>
<summary> Download pre-built config made by others </summary>

```sh
cr download https://github.com/cargo-runner/configs/raw/main/leptos/leptos.toml
# you can also download a config and set it as default config for specific context
cr download https://github.com/cargo-runner/configs/raw/main/leptos/leptos.toml --default run
```

</details>

<details>
<summary> Generate config</summary>

```sh
# pass an optional name 
cr generate
# if you pass the name it would generate a config for the given name if it exists
cr generate leptos
# by default it would be generated on current working directory
# if we want to generate on a different dir we can pass --dir
cr generate --dir ~/.cargo-runner/configs/leptos leptos
# to download a config and generate it on current working directory
cr generate --url https://github.com/cargo-runner/configs/raw/main/leptos/leptos.toml
# if you want to download and set different dir name you can also pass --dir
cr generate --url https://github.com/cargo-runner/configs/raw/main/leptos/leptos.toml --dir ~/.cargo-runner/configs/leptos
```

[example-override.toml](./cargo-runner-leptos.toml) generated 

```toml
[run]
default = "leptos"

[[run.commands]]
name = "leptos"
command_type = "sub_command"
command = "leptos"
sub_command = "watch"
allowed_subcommands = []

[run.commands.env]
```

</details>


<details>
<summary>Validate custom config , auto-generate backup if invalid format is used</summary>

```sh
# if you dont pass in a path it would assume it is on current working directory
cr validate
# you can pass a config file path
cr validate ~/.cargo-runner/configs/leptos/leptos.toml
# if you need to validate default config you can pass --default
cr validate --default
```

Invalid config would move the file to e.g. `$name.0.bak` and a valid config would be generated for you to modify.


</details>

<details>
<summary> Change Default for Specific Context </summary>

```sh
# the name params is optional
cr default run
# if the name if provided it would check if the context exists
# before setting it as default , if it doesnt exist nothing would happen
# and an error would be shown
cr default run leptos
```

</details>

<details>

<summary>Override Parameters for a specific context</summary>

```sh
cr params $context --path $path
# example
cr params run --path /Users/uriah/oss/rx/crates/cli/src/main.rs
```

Note: The file path here would be used to determine where to look for the `cargo-runner.toml` file, and update the `default` context with the parameters.






#### b. Cargo Builder CLI

> This features comes with `cargo-runner-cli`

<details>
<summary>Generate Commands for a specific context</summary>

```sh
cr build $context --path $path --ln $ln --col $col
```

Note: This would read the file , and current position if --ln and --col are provided, it would use that to determine the current context. and would use the nearest `cargo-runner.toml` file near `Cargo.toml` to generate the commands. It would use as well the `Cargo.toml` file to add other metadata like `package` name or `bin` name or `features` when generating commands.

</details>




## VsCode Extension (Cargo Runner)

### Features

#### a. Config Builder on Vscode

- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Cargo Runner: Init Config`  to Generate Default `Config` on `~/.cargo-runner/config.toml`
- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Cargo Runner: Generate Config`  to Generate a `CommandConfig`. You can either type a name or pass a url to download a config.
- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Cargo Runner: Download Config` to Download a `CommandConfig` from a url.
- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Cargo Runner: Set Default Context` to choose a context from list: `run` , `build`, `test`, and `bench`.
- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Cargo Runner: Validate Config` to Validate a `CommandConfig` file.

Note: The default directory when running any of the commands above would either be the nearest `Cargo.toml` file on the current open file or workspace root.

#### b. Cargo Builder on Vscode
- Uses your default `config` or `cargo-runner.toml` file to generate commands for all `command_type` variants.
- Auto appends or prepends `sub_command, parameters or options` for **command_type** variants of  `cargo` or `sub_command` when executing a command.
- Reads your `Cargo.toml` file get other metadata like `package` name or `bin` name or `features` when generating commands.

#### c. Cargo Runner on Vscode

- Press <kbd>CMD</kbd> + <kbd>R</kbd> (default) or bind to a key of your choice to execute any command.

Note: Prior Executing commands , it would check for the current context e.g. the open file is **main.rs** it would do further check if the context is either `run` or `test`. Once the context is found it would use the `default` **CommandConfig** for that current context , and build the correct command to execute.

> Under the hood it uses the CargoBuilder to smartly build the command to execute or override the parameters.

#### d. Override Parameters on default Context on VSCode

- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>R</kbd> to `Cargo Runner: Override Parameters` 

Note: This would only work if a rust file is open, and would look for current context e.g. the open file is **main.rs** it would do further check if the context is either `run` or `test`. It would update the `cargo-runner.toml` next to `Cargo.toml` using the current context , it would update the current context `default` **CommandConfig** 



