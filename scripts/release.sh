#!/usr/bin/env bash
set -e

cargo install cargo-release -f
git remote set-url origin https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git
git fetch -p
git checkout $TRAVIS_BRANCH
git pull --tags origin $TRAVIS_BRANCH
cargo login $CRATES_TOKEN
export PATH="$PATH:$HOME/.cargo/bin"
cargo release --no-dev-version --no-confirm patch
