#!/usr/bin/env bash
set -e

cargo doc -q --color never
cargo package --color never
