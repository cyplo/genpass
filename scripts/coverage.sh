#!/usr/bin/env bash

set -e
cargo tarpaulin --out Xml
bash <(curl -s https://codecov.io/bash)
