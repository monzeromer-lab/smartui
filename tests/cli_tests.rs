use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;

#[test]
fn test_cli_without_api_key() {
    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("explain")
        .arg("ls -la")
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("explain"))
        .stdout(predicate::str::contains("summarize"))
        .stdout(predicate::str::contains("translate"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_invalid_command() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_cli_explain_without_command() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("explain")
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"))
        .stderr(predicate::str::contains("<COMMAND>"));
}

#[test]
fn test_cli_fix_typos_without_text() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("fix-typos")
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"))
        .stderr(predicate::str::contains("<TEXT>"));
}

#[test]
fn test_cli_summarize_with_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "This is a test text to summarize.").unwrap();

    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("summarize")
        .arg(file_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_summarize_with_stdin() {
    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("summarize")
        .arg("--stdin")
        .write_stdin("This is a test text to summarize.")
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_translate() {
    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("translate")
        .arg("Hello")
        .arg("--to")
        .arg("fr")
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_define() {
    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("define")
        .arg("API")
        .arg("--context")
        .arg("programming")
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_list_dir() {
    let dir = tempdir().unwrap();
    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("list-dir")
        .arg(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_explain_code() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "fn main() {{ println!(\"Hello, World!\"); }}").unwrap();

    env::remove_var("GEMINI_API_KEY");
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("explain-code")
        .arg(file_path)
        .arg("--language")
        .arg("Rust")
        .assert()
        .failure()
        .stderr(predicate::str::contains("ApiKeyNotFound"));
}

#[test]
fn test_cli_summarize_without_input() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("summarize")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Either --stdin or a file path must be provided"));
}

#[test]
fn test_cli_translate_without_text() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("translate")
        .arg("--to")
        .arg("fr")
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"))
        .stderr(predicate::str::contains("<TEXT>"));
}

#[test]
fn test_cli_define_without_term() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("define")
        .arg("--context")
        .arg("programming")
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"))
        .stderr(predicate::str::contains("<TERM>"));
}

#[test]
fn test_cli_explain_code_without_file() {
    let mut cmd = Command::cargo_bin("smartui").unwrap();
    cmd.arg("explain-code")
        .arg("--language")
        .arg("Rust")
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"))
        .stderr(predicate::str::contains("<FILE>"));
} 