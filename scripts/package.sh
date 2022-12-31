#!/usr/bin/env bash
set -e

cargo doc --color never
cargo package --color never
