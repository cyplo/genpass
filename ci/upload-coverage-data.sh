#!/usr/bin/env bash

set -e

pushd `pwd`
mkdir -p target/coverage
cd target/coverage
wget -c https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
cd kcov-master
mkdir build
cd build
cmake ..
make
make install DESTDIR=../../kcov-build
popd
for file in target/debug/genpass-*[^\.d]; do
    echo "processing $file"
    mkdir -p "target/cov/$(basename $file)"
    ./target/coverage/kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done
bash <(curl -s https://codecov.io/bash)
echo "Uploaded code coverage"
