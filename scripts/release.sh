#!/usr/bin/env bash
set -e

cargo install cargo-release -f
export PATH="$PATH:$HOME/.cargo/bin"
cargo release --no-dev-version --no-confirm patch
