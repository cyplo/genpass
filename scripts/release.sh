#!/usr/bin/env bash
set -e

echo "starting to publish"

export PATH="$PATH:$HOME/.cargo/bin"
mkdir -p ~/.ssh
ssh-keyscan git.cyplo.dev >> ~/.ssh/known_hosts

git config user.email "release-bot@cyplo.dev"
git config user.name "Release Bot"
git config init.defaultBranch main
git remote set-url origin gitea@git.cyplo.dev:cyplo/genpass.git
git fetch
git checkout main
git pull origin main --tags

if [ $(git tag --points-at HEAD | wc -m) -ne 0 ]; then
    echo "skipping publish as already on a tag"
    exit 0
fi

cargo release --no-confirm --no-push --no-publish --execute patch
git push origin --all
git push origin --tags

export CARGO_NET_RETRY=16
export CARGO_HTTP_MULTIPLEXING=false

export ALL_PROXY=socks5://localhost:1055/
export HTTP_PROXY=http://localhost:1055/
export http_proxy=http://localhost:1055

cargo publish --color never
