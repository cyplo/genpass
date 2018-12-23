#!/usr/bin/env bash
set -e

(test -x "${HOME}/.cargo/bin/cargo-release" || cargo install cargo-release)
(test -x "${HOME}/.cargo/bin/cargo-install-update" || cargo install cargo-update)
cargo install-update -a

rustup component add clippy --toolchain stable
rustup component add rustfmt --toolchain stable

rustup install nightly
RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo +nightly install --force cargo-tarpaulin
