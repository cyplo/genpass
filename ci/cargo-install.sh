#!/usr/bin/env bash
set -e

cargo install --force cargo-release
rustup install nightly
RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo +nightly install --force cargo-tarpaulin
