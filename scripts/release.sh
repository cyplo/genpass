#!/usr/bin/env bash
set -e

cargo install cargo-release -f
export PATH="$PATH:$HOME/.cargo/bin"
ssh-keyscan git.sr.ht >> ~/.ssh/known_hosts
git config user.email "releases@cyplo.dev"
git config user.name "Release Bot"
git checkout master
cargo release --no-dev-version --no-confirm patch
