# Change Log

All notable changes to the "cargo-runner" extension will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.
## 2.0.0
- Use Rust WASM Component Model
- Use Cargo workspace
- Config Builder
- Auto backup misconfigured config file

## TODO:

### convert the following methods to rust implementation if possible

- [ ] ** is_integration_test **
- [ ] ** is_workspace **
- [ ] ** is_inside_mod_test **
- [ ] ** is_inside_examples **
- [ ] ** is_file_in_test_context **
- [ ] ** is_cargo_nextest_installed **
- [ ] ** get_test_fn_name **
- [ ] ** get_package_name **
- [ ] ** get_cargo_toml (nearest) **
- [ ] ** get_cargo_runner_toml (nearest) **
- [ ] ** get_bin_name **
- [ ] ** get_benchmark **
- [ ] ** check_crate_type **

### CommandBuilder 
- [ ] use custom validator 
- [ ] use config to build commands
- [ ] use Cargo.toml to build commands
- [ ] use well known conditions to set other params or options e.g. `lib.rs` etc.
- [ ] use config load to get config