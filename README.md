# Cargo Runner

A Vscode plugin for 10X Rust Developer Tool to Run, Build, or Test without Mental Overhead.

## Techstacks

- TypeScript
- Rust
- WASM
- WIT (WebAssembly Interface Types)


## Features

### 1. Config Builder

- Generate Default config on `~/.cargo-runner/config.toml`
- Support different context like `run` , `build` , `test` , `bench` and `debug`
- Override default `config` by using custom `cargo-runner.toml` file see [example-override.toml](./example-override.toml)
- Wrong configuration would auto backup file e.g. `cargo-runner.0.bak`, and a correct config would be generated for you to modify.

### 2. Config Manager

- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Override Config`  to Override a `CommandConfig`
- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Generate Config` to  Generate prebuilt config for commonly used frameworks e.g. `leptos` , `dioxus` etc. relative to the current file. or workspace.
- Press <kbd>CMD</kbd> + <kbd>SHIFT</kbd> + <kbd>P</kbd> and type `Set Default Config` to change default `CommandConfig` for choosen context e.g. `run`.
- Multiple config per crate , works on rust workspaces

### 3. Cargo Builder

- Auto appends or prepends `sub_command, parameters or options` for **command_type** of `cargo` or `sub_command` when executing a command.

### 4. Cargo Runner

- Press <kbd>CMD</kbd> + <kbd>R</kbd> (default) or bind to a key of your choice to execute any command.
- smart context command runner it would run the correct command based on the current cursor or file context.
