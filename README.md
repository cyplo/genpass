# genpass [![Build Status](https://travis-ci.org/cyplo/genpass.svg?branch=master)](https://travis-ci.org/cyplo/genpass) [![Donate](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/cyplo/donate)

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
$ genpass --help

genpass 0.3.0
A simple yet robust commandline random password generator.

USAGE:
  genpass [FLAGS] [OPTIONS]

FLAGS:
  -h, --help                 Prints help information
  -d, --include-digit        Include at least one digit
  -l, --include-lowercase    Include at least one lowercase letter
  -s, --include-special      Include at least one special (non-alphanumericc) character
  -u, --include-uppercase    Include at least one uppercase letter
      --version

OPTIONS:
      --length <length>    The length of the password to generate [default: 32] 
```

### A note on passphrases
* Passphrases are generated using [EFF's](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) "long" password list.
* Passphrases still abide to the overall length requirements (this may result in the last word of the passphrase being cut to length)

## A road to 1.0

TODOs to get `genpass` to 1.0

* [support custom sets of characters](https://github.com/cyplo/genpass/issues/4)
* [add benchmarks](https://github.com/cyplo/genpass/issues/5)

## Contributing
All contributions welcome !
Ideally - start a discussion with an issue first before contributing a PR.
