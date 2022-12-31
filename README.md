# genpass [![status-badge](https://ci.cyplo.dev/api/badges/cyplo/genpass/status.svg)](https://ci.cyplo.dev/cyplo/genpass) [![builds.sr.ht status](https://builds.sr.ht/~cyplo/genpass.svg)](https://builds.sr.ht/~cyplo/genpass?) [![dependency status](https://deps.rs/repo/github/cyplo/genpass/status.svg)](https://deps.rs/repo/github/cyplo/genpass) [![Donate](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/cyplo/donate)

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

### Generating a password directly into your system clipboard

On a Mac:
```
genpass | pbcopy
```
On Linux:
```
genpass | xclip -selection clipboard
```

## Getting genpass

### If you're using Nix flakes
* you can try genpass without installing it `nix run git+https://git.cyplo.dev/cyplo/genpass.git`
* flake url is `https://git.cyplo.dev/cyplo/genpass.git`

### On any system with Rust installed

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
-l, --include-lowercase    Generate the password using lowercase letters
-n, --include-numeric      Generate the password using numeric characters
-s, --include-special      Generate the password using special (non-alphanumeric) characters
-u, --include-uppercase    Generate the password using uppercase letters
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

### Sources

`git clone https://git.cyplo.dev/cyplo/genpass.git`

### quickstart

* you can use [Nix](https://nixos.org/download.html) to recreate the development environment reproducibly
* `nix develop` in this repo will give you a shell with all the dependencies needed

