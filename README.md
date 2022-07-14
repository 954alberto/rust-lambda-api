# AWS LAMBDA API USER MANAGEMENT

## SETUP OF THE PROJECT

### HOW TO BUILD THE BINARY
As explained in the documentation <https://doc.rust-lang.org/cargo/reference/config.html>

I use a Cargo configuration file to specify: a default target-triple for your project and the necessary environment variables for the build.

In your project's root, create a .cargo directory and a config.toml file in it with the following contents:

```toml
# <project_folder>/.cargo/config.toml
[build]
target = "aarch64-unknown-linux-gnu"

[env]
CC_aarch64_unknown_linux_gnu = "aarch64-unknown-linux-gnu-gcc"
CXX_aarch64_unknown_linux_gnu = "aarch64-unknown-linux-gnu-g++"
AR_aarch64_unknown_linux_gnu = "aarch64-unknown-linux-gnu-ar"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-unknown-linux-gnu-gcc"
```

#### ON MAC
```bash
rustup target add aarch64-unknown-linux-gnu

brew tap messense/macos-cross-toolchains                                                                                                                ─╯
brew install aarch64-unknown-linux-gnu

cargo build --release
```