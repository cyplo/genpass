#!/usr/bin/env bash
set -e

cargo pants

cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check
cargo test
cargo test -- --ignored
cargo build --release
./target/release/genpass --version
