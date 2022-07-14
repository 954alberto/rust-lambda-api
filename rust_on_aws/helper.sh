LAMBDA_ARCH="linux/arm64"
RUST_TARGET="aarch64-unknown-linux-gnu"
RUST_VERSION="1.60"
PROJECT_NAME="rust_on_aws"

dockercompile() {
  docker run --platform ${LAMBDA_ARCH} \
    --rm --user "$(id -u)":$(id -g) \
    -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
    cargo build --release --target ${RUST_TARGET}
}

crosscompile() {

#  brew tap messense/macos-cross-toolchains
#  brew install aarch64-unknown-linux-gnu
#
#  export CC_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-gcc
#  export CXX_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-g++
#  export AR_aarch64_unknown_linux_gnu=aarch64-unknown-linux-gnu-ar
#  export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

  cargo build --release --target ${RUST_TARGET}
}

zipRustLambda() {
  cd ./target/${RUST_TARGET}/release/${PROJECT_NAME} \
    && zip lambda.zip bootstrap \
    && rm bootstrap
}