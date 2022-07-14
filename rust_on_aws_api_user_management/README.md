# AWS LAMBDA API USER MANAGEMENT

## SETUP OF THE PROJECT

### HOW TO BUILD THE BINARY
As explained in the documentation <https://doc.rust-lang.org/cargo/reference/config.html>

You could use a Cargo configuration file to specify a default target-triple for your project. 
In your project's root, create a .cargo directory and a config file in it with the following contents:

```
// <project_folder>/.cargo/config
[build]
target = "aarch64-unknown-linux-gnu"
```

#### ON MAC
```bash

rustup target add aarch64-unknown-linux-gnu

brew tap messense/macos-cross-toolchains                                                                                                                ─╯
brew install aarch64-unknown-linux-gnu

export CC_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-g++
export AR_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

cargo build --release

```