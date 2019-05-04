# genpass [![Build Status](https://travis-ci.org/cyplo/genpass.svg?branch=master)](https://travis-ci.org/cyplo/genpass) [![Donate](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/cyplo/donate)

A simple yet robust commandline random password generator.

Multiplatform (Linux, Mac, Windows).
Fast.

You can use it generate keys or passwords in scripts or use it as your primary desktop password generator.

Has extensive generative test suite, including tests against a [Rust port](https://crates.io/crates/zxcvbn) of Dropbox's password strength tester [`zxcvbn`](https://www.usenix.org/conference/usenixsecurity16/technical-sessions/presentation/wheeler)

## Typical usage
```
genpass                  # use defaults, they're good
genpass 2048             # generate long password, can be used as a key
genpass --passphrase 128 # generate longer passphrase
genpass -dlu             # no special characters
```

### Generating a password directly into your system cliboard

On a Mac:
```
genpass | pbcopy
```
On Linux:
```
genpass | xclip -selection clipboard
```


## Installation
On a system with [Rust](https://www.rust-lang.org/en-US/) installed:
```
$ cargo install genpass
```

## Commandline options
```
$ genpass --help

USAGE:
    genpass [FLAGS] [length]

FLAGS:
    -h, --help                 Prints help information
    -d, --include-digit        Include at least one digit
    -l, --include-lowercase    Include at least one lowercase letter
    -s, --include-special      Include at least one special (non-alphanumeric) character
    -u, --include-uppercase    Include at least one uppercase letter
        --passphrase           Create a passphrase of (at least) the given length instead of a password.
        --version

ARGS:
    <length>    The length of the password to generate [default: 32]
```

### A note on passphrases
* Passphrases are generated using [EFF's](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) "long" password list.
* Passphrases are at least `--length` characters long, not necessarily exactly that long.

## A road to 1.0

TODOs to get `genpass` to 1.0

* [support custom sets of characters](https://github.com/cyplo/genpass/issues/4)
* [add benchmarks](https://github.com/cyplo/genpass/issues/5)

## Contributing
All contributions welcome !
Ideally - start a discussion with an issue first before contributing a PR.
