#!/usr/bin/env bash

if [[ ! -x `which cargo-release` ]]; then
    cargo install cargo-release
fi

if [[ ! -x `which cargo-tarpaulin` ]]; then
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
fi
