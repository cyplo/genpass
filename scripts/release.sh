#!/usr/bin/env bash
set -e

if [ $(git tag --points-at HEAD | wc -m) -ne 0 ]; then
    echo "skipping publish as already on a tag"
    exit 0
fi

export PATH="$PATH:$HOME/.cargo/bin"
mkdir -p ~/.ssh
ssh-keyscan git.cyplo.dev >> ~/.ssh/known_hosts

git config user.email "releases@cyplo.dev"
git config user.name "Release Bot"
git config init.defaultBranch main
git checkout main
git remote set-url origin gitea@git.cyplo.dev:cyplo/genpass.git
cargo release --no-confirm --no-push --no-publish --execute patch
git push origin --all
git push origin --tags
cargo publish
