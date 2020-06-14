#!/usr/bin/env bash
set -e

if [[ ! -z $CI ]]; then
    export CARGO_HUSKY_DONT_INSTALL_HOOKS=true
fi

cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check
cargo test
cargo test -- --ignored
