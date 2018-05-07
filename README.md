# genpass [![Build Status](https://travis-ci.org/cyplo/genpass.svg?branch=master)](https://travis-ci.org/cyplo/genpass)

A simple yet robust commandline random password generator.

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
    -u               Include at least one uppercase letter
    -V, --version    Prints version information

ARGS:
    <length>     [default: 32]
```
