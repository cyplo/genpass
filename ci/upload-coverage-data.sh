#!/usr/bin/env bash

set -e

if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml
    bash <(curl -s https://codecov.io/bash)
fi
