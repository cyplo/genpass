#!/usr/bin/env bash
set -e

rustup component add clippy --toolchain stable
rustup component add rustfmt --toolchain stable

(test -x "${HOME}/.cargo/bin/cargo-release" || cargo install cargo-release)
(test -x "${HOME}/.cargo/bin/cargo-install-update" || cargo install cargo-update)
(test -x "${HOME}/.cargo/bin/cargo-pants" || cargo install cargo-pants)

cargo install-update -a
