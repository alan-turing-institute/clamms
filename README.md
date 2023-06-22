# CLAMMS

## Quickstart
- Clone repo:
  ```bash
  git clone https://github.com/alan-turing-institute/clamms/
  ```
- Install:
  - [Rust](https://www.rust-lang.org/tools/install)
  - [cargo-make](https://crates.io/crates/cargo-make#installation)
- Run a simulation from repo root (see [config](clamms-config.toml) for parameters to vary):
  - Without visualization:
  ```bash
  cargo run --release
  ```
  - With visualization:
  ```bash
  cargo make run --release
  ```

## Config file

The main config file is `clamms-config.toml`. If the environment variable `CLAMMS_CONFIG` is set, it will be used as the path to the config file. If not, it will look for `clamms-config.toml` in the root of the repo.
