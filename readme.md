# Cargo Runner

## Features

1. Config Builder
- Config::init() is Infallible
- Config::load() when parsing invalid toml file generates `$name.$number.bak` 
- When parsing toml file with invalid data, it gets replace with default toml file
- Config::merge(default,other) allows merging two config and override the previous one.
- Invalid `default` name on any context would fallback to default


## TODO:

1. CommandBuilder
- When generating complex commands we would use the Config to build the command output

2. CommandBuilder Validation using Config e.g. `allowed_subcommands` and `allowed_options`

3. Add extra fields on CommandConfig

- release (bool)
- profile (String) (read from Cargo.toml or must be from default cargo profile)
- target (we should have list of possible target)
- features (--features that are read on Cargo.toml)
- toolchain (nightly ,stable)
- allowed_options (all the -- options in commands must be listed here so we can use it as validation on command builder to check if all optons has match on allowed_options)
- params (this should replace our override commands on cargo-runner original impl when pressing CMD+SHIFT+R)

4. CommandBuilder  Auto Generation of Params/Options  using the cwd, filepath , ln / col and Cargo.toml

Note: we can auto add some of the params like 
Its either Read from ./Cargo.toml or workspace Cargo.toml or if it matches criteria like src/bin/*.rs with main fn
/src/lib.rs , src/main.rs


1. Run:

> Usage: cargo run [OPTIONS] [ARGS]...

<details> 

<summary> OPTIONS </summary>

```sh
Package Selection:
  -p, --package [<SPEC>]  Package with the target to run

Target Selection:
      --bin [<NAME>]      Name of the bin target to run
      --example [<NAME>]  Name of the example target to run

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
```

</details>

2. Build

> Usage: cargo build [OPTIONS]

<details> 

<summary> OPTIONS </summary>

```sh
Package Selection:
  -p, --package [<SPEC>]  Package to build (see `cargo help pkgid`)
      --workspace         Build all packages in the workspace
      --exclude <SPEC>    Exclude packages from the build
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Build only this package's library
      --bins              Build all binaries
      --bin [<NAME>]      Build only the specified binary
      --examples          Build all examples
      --example [<NAME>]  Build only the specified example
      --tests             Build all test targets
      --test [<NAME>]     Build only the specified test target
      --benches           Build all bench targets
      --bench [<NAME>]    Build only the specified bench target
      --all-targets       Build all targets

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
```

</details>


3. Test:

> Usage: cargo test [OPTIONS] [TESTNAME] [-- [ARGS]...]
> Usage: cargo nextest run [OPTIONS] [FILTERS]... [-- <FILTERS_AND_ARGS>...]

Note: There are difference with default cargo test and cargo-nextest

<details> 

<summary> OPTIONS </summary>

```sh
Package Selection:
  -p, --package [<SPEC>]  Package to run tests for
      --workspace         Test all packages in the workspace
      --exclude <SPEC>    Exclude packages from the test
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Test only this package's library
      --bins              Test all binaries
      --bin [<NAME>]      Test only the specified binary
      --examples          Test all examples
      --example [<NAME>]  Test only the specified example
      --tests             Test all test targets
      --test [<NAME>]     Test only the specified test target
      --benches           Test all bench targets
      --bench [<NAME>]    Test only the specified bench target
      --all-targets       Test all targets (does not include doctests)
      --doc               Test only this library's documentation

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
```

</details>


4. Bench: (bench name should be parse from eithe the fn , or file)

> Usage: cargo bench [OPTIONS] [BENCHNAME] [-- [ARGS]...]

<details> 

<summary> OPTIONS </summary>

```sh
Package Selection:
  -p, --package [<SPEC>]  Package to run benchmarks for
      --workspace         Benchmark all packages in the workspace
      --exclude <SPEC>    Exclude packages from the benchmark
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Benchmark only this package's library
      --bins              Benchmark all binaries
      --bin [<NAME>]      Benchmark only the specified binary
      --examples          Benchmark all examples
      --example [<NAME>]  Benchmark only the specified example
      --tests             Benchmark all test targets
      --test [<NAME>]     Benchmark only the specified test target
      --benches           Benchmark all bench targets
      --bench [<NAME>]    Benchmark only the specified bench target
      --all-targets       Benchmark all targets

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature
```

</details>