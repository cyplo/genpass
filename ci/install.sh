#!/usr/bin/env bash
set -e

rustup install nightly

(test -x "${HOME}/.cargo/bin/cargo-release" || cargo install cargo-release)
(test -x "${HOME}/.cargo/bin/cargo-install-update" || cargo install cargo-update)

set +e
cargo install-update -a
set -e
rustup run nightly cargo install-update -a

rustup component add clippy --toolchain stable
rustup component add rustfmt --toolchain stable
