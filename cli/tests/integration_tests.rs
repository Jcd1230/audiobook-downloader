use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("audiobook-downloader").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A fast, modular CLI"));
}

#[test]
fn test_list_no_library() {
    // We point the config dir to a non-existent temp dir to simulate empty state
    let mut cmd = Command::cargo_bin("audiobook-downloader").unwrap();
    cmd.arg("list")
        .env("XDG_CONFIG_HOME", "/tmp/non-existent-dir-12345");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No books in local library"));
}
