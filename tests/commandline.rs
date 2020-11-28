#[allow(dead_code)]
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn no_arguments() {
    let mut cmd = Command::cargo_bin(built_info::PKG_NAME).unwrap();
    cmd.assert().success();
}

#[test]
fn no_stderr() {
    let mut cmd = Command::cargo_bin(built_info::PKG_NAME).unwrap();
    cmd.assert().stderr(predicate::str::is_empty());
}

#[test]
fn some_stdout() {
    let mut cmd = Command::cargo_bin(built_info::PKG_NAME).unwrap();
    cmd.assert().stdout(predicate::str::is_empty().not());
}
