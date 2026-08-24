use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io;
use tempfile::tempdir;

fn tree_create() -> Command {
    Command::cargo_bin("tree-create").expect("tree-create binary should build")
}

#[test]
fn help_describes_input_modes_and_safety_flags() {
    tree_create()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: tree-create"))
        .stdout(predicate::str::contains("--inline"))
        .stdout(predicate::str::contains("--force"))
        .stdout(predicate::str::contains("--dry-run"));
}

#[test]
fn file_input_creates_tree_in_working_directory() -> io::Result<()> {
    let dir = tempdir()?;
    let input = dir.path().join("tree.txt");
    fs::write(&input, "project/\n  src/\n    main.rs")?;

    tree_create()
        .current_dir(dir.path())
        .arg(&input)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created directory"));

    assert!(dir.path().join("project/src/main.rs").is_file());
    Ok(())
}

#[test]
fn inline_input_reads_stdin_and_creates_tree() -> io::Result<()> {
    let dir = tempdir()?;

    tree_create()
        .current_dir(dir.path())
        .arg("--inline")
        .write_stdin("project/\n  README.md\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Start typing below"));

    assert!(dir.path().join("project/README.md").is_file());
    Ok(())
}

#[test]
fn dry_run_reports_actions_without_creating_tree() -> io::Result<()> {
    let dir = tempdir()?;
    let input = dir.path().join("tree.txt");
    fs::write(&input, "project/\n  README.md")?;

    tree_create()
        .current_dir(dir.path())
        .arg("--dry-run")
        .arg(&input)
        .assert()
        .success()
        .stdout(predicate::str::contains("dry-run Would create directory"))
        .stdout(predicate::str::contains("dry-run Would create file"));

    assert!(!dir.path().join("project").exists());
    Ok(())
}

#[test]
fn conflicting_input_modes_fail_with_usage_error() {
    tree_create()
        .args(["--inline", "tree.txt"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn invalid_tree_fails_without_creating_root() -> io::Result<()> {
    let dir = tempdir()?;
    let input = dir.path().join("invalid.txt");
    fs::write(&input, "project\n  README.md")?;

    tree_create()
        .current_dir(dir.path())
        .arg(&input)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "must be a directory ending with '/'",
        ));

    assert!(!dir.path().join("project").exists());
    Ok(())
}
