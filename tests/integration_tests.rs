use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_cli_functionality() -> std::io::Result<()> {
    let temp_dir = TempDir::new()?;
    let input_file = temp_dir.path().join("input.txt");
    fs::write(&input_file, "
test_cli_project/
    src/
        main.rs
    Cargo.toml
")?;

    let output = Command::new(env!("CARGO_BIN_EXE_tree-create"))
        .arg(input_file)
        .current_dir(temp_dir.path())
        .output()?;

    assert!(output.status.success());
    
    assert!(temp_dir.path().join("test_cli_project").is_dir());
    assert!(temp_dir.path().join("test_cli_project/src").is_dir());
    assert!(temp_dir.path().join("test_cli_project/src/main.rs").is_file());
    assert!(temp_dir.path().join("test_cli_project/Cargo.toml").is_file());

    Ok(())
}
