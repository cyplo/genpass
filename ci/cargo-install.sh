#!/usr/bin/env bash
set -e

cargo install --force cargo-release
RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install --force cargo-tarpaulin
