# genpass [![Build Status](https://travis-ci.org/cyplo/genpass.svg?branch=master)](https://travis-ci.org/cyplo/genpass)

A simple yet robust commandline random password generator.

Multiplatform (Linux, Mac, Windows).
Fast.

Has extensive generative test suite, including tests against a [Rust port](https://crates.io/crates/zxcvbn) of Dropbox's password strength tester [`zxcvbn`](https://www.usenix.org/conference/usenixsecurity16/technical-sessions/presentation/wheeler)

## Installation
On a system with [Rust](https://www.rust-lang.org/en-US/) installed:
```
$ cargo install genpass
```

## Usage
```
$ genpass -h

USAGE:
    genpass [FLAGS] [length]

FLAGS:
    -h, --help       Prints help information
    -d               Include at least one digit
    -l               Include at least one lowercase letter
    -s               Include at least one (non alphanumeric) special character
    -u               Include at least one uppercase letter
    -V, --version    Prints version information

ARGS:
    <length>     [default: 32]
```

## A road to 1.0

TODOs to get `genpass` to 1.0

* [support custom sets of characters](https://github.com/cyplo/genpass/issues/4)
* [add benchmarks](https://github.com/cyplo/genpass/issues/5)

## Contributing
All contributions welcome !
Ideally - start a discussion with an issue first before contributing a PR.