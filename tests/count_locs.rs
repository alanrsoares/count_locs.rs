use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use std::fs;

#[test]
fn it_calculates_total_loc_for_single_glob() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test files
    fs::write(temp_path.join("file1.rs"), "fn main() {}\n").unwrap();
    fs::write(temp_path.join("file2.rs"), "fn test() {}\nfn another() {}\n").unwrap();

    // Run the command
    Command::cargo_bin("count_locs")
        .unwrap()
        .args([temp_path.to_str().unwrap(), "**/*.rs"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Total lines of code: 3"));
}

#[test]
fn it_displays_breakdown_for_multiple_globs() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create test files
    fs::write(temp_path.join("file1.ts"), "let x = 42;\n").unwrap();
    fs::write(temp_path.join("file2.tsx"), "const y = () => {}\nconsole.log(y);\n").unwrap();
    fs::write(temp_path.join("file3.rs"), "fn main() {}\nfn helper() {}\n").unwrap();

    // Run the command
    Command::cargo_bin("count_locs")
        .unwrap()
        .args([
            temp_path.to_str().unwrap(),
            "**/*.ts",
            "**/*.tsx",
            "**/*.rs",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Breakdown of Lines of Code by Glob:"))
        .stdout(predicate::str::contains("**/*.ts: 1"))
        .stdout(predicate::str::contains("**/*.tsx: 2"))
        .stdout(predicate::str::contains("**/*.rs: 2"))
        .stdout(predicate::str::contains("Total lines of code: 5"));
}

#[test]
fn it_handles_no_matching_files() {
    let temp_dir = tempdir().expect("failed to create temp dir");
    let temp_path = temp_dir.path();

    // Run the command with a glob that matches nothing
    Command::cargo_bin("count_locs")
        .unwrap()
        .args([temp_path.to_str().unwrap(), "**/*.java"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Total lines of code: 0"));
}

#[test]
fn it_errors_with_invalid_usage() {
    Command::cargo_bin("count_locs")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}
