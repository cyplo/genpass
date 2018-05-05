#!/usr/bin/env bash

if [[ ! -x `which cargo-release` ]]; then
    cargo install cargo-release
fi
