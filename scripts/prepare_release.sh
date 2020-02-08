#!/usr/bin/env bash
set -e

git checkout $TRAVIS_BRANCH
git remote set-url origin https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git
cargo login $CRATES_TOKEN
cargo release --no-dev-version --no-confirm patch
