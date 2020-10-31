#!/usr/bin/env bash
set -e

cargo install cargo-release -f
export PATH="$PATH:$HOME/.cargo/bin"
cargo login $CRATES_TOKEN
