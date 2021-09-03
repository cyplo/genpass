#!/usr/bin/env bash
set -e

if [ $(git tag --points-at HEAD | wc -m) -ne 0 ]; then
    echo "skipping publish as already on a tag"
    exit 0
fi

cargo install cargo-release -f
export PATH="$PATH:$HOME/.cargo/bin"
ssh-keyscan git.sr.ht >> ~/.ssh/known_hosts
git config user.email "releases@cyplo.dev"
git config user.name "Release Bot"
git checkout master
cargo release --no-dev-version --no-confirm --execute patch
