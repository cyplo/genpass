#!/usr/bin/env bash

set -e

if [[ "$TRAVIS_RUST_VERSION" == "stable" ]]; then
    bash <(curl -s https://codecov.io/bash)
fi
