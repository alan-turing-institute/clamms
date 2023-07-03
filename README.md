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

## Running on VM with GPU
First install the latest version of CUDA (currently 12.1). To install Rust with tch-rs support, first install PyTorch v2.0.0 (you may need to install pip first):

`pip3 install torch==2.0.0 torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118`

To install without GPU support:

`pip install torch==2.0.0 torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu`

Now install Rust:

`curl https://sh.rustup.rs -sSf | sh -s -- -y`

Set some environment variables to point Rust towards libtorch:

`export LIBTORCH_USE_PYTORCH=1`

`export LD_LIBRARY_PATH=/home/azureuser/.local/lib/python3.10/site-packages/torch/lib:$LD_LIBRARY_PATH`

These have been added to `/home/azureuser/.bashrc` on both Azure VMs.
