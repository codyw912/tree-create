use std::fs;
use std::io;
use tempfile::tempdir;
use tree_create::create_tree;

/// Helper function to verify directory structure
fn verify_structure(dir: &std::path::Path, expected_paths: &[&str]) -> io::Result<()> {
    for path in expected_paths {
        let full_path = dir.join(path);
        assert!(full_path.exists(), "Path not found: {:?}", full_path);

        // Verify if it's the right type (file or directory)
        if path.ends_with('/') {
            assert!(
                full_path.is_dir(),
                "Expected directory, found file: {:?}",
                full_path
            );
        } else {
            assert!(
                full_path.is_file(),
                "Expected file, found directory: {:?}",
                full_path
            );
        }
    }
    Ok(())
}

#[test]
fn test_create_from_ascii_tree() -> io::Result<()> {
    let dir = tempdir()?;

    let input = "\
project/
├── src/
│   ├── main.rs
│   └── lib.rs
└── Cargo.toml";

    create_tree(input, dir.path(), false, false)?;

    verify_structure(
        dir.path(),
        &[
            "project/",
            "project/src/",
            "project/src/main.rs",
            "project/src/lib.rs",
            "project/Cargo.toml",
        ],
    )
}

#[test]
fn test_create_from_indented() -> io::Result<()> {
    let dir = tempdir()?;

    let input = "\
project/
  src/
    main.rs
    lib.rs
  Cargo.toml";

    create_tree(input, dir.path(), false, false)?;

    verify_structure(
        dir.path(),
        &[
            "project/",
            "project/src/",
            "project/src/main.rs",
            "project/src/lib.rs",
            "project/Cargo.toml",
        ],
    )
}

#[test]
fn test_deep_nesting() -> io::Result<()> {
    let dir = tempdir()?;

    let input = "\
project/
  src/
    components/
      ui/
        buttons/
          primary.rs
          secondary.rs
        inputs/
          text.rs
          number.rs";

    create_tree(input, dir.path(), false, false)?;

    verify_structure(
        dir.path(),
        &[
            "project/",
            "project/src/",
            "project/src/components/",
            "project/src/components/ui/",
            "project/src/components/ui/buttons/",
            "project/src/components/ui/buttons/primary.rs",
            "project/src/components/ui/buttons/secondary.rs",
            "project/src/components/ui/inputs/",
            "project/src/components/ui/inputs/text.rs",
            "project/src/components/ui/inputs/number.rs",
        ],
    )
}

#[test]
fn test_empty_input() {
    let dir = tempdir().unwrap();
    let result = create_tree("", dir.path(), false, false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Input is empty");
}

#[test]
fn test_indented_root_error() {
    let dir = tempdir().unwrap();
    let input = "  project/\n  src/";

    let result = create_tree(input, dir.path(), false, false);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Root directory (line 1) should not be indented"
    );
}

#[test]
fn test_leading_empty_lines_are_ignored() -> io::Result<()> {
    let dir = tempdir()?;

    create_tree("\n \t\nproject/\n  src/", dir.path(), false, false)?;

    assert!(dir.path().join("project/src").is_dir());
    Ok(())
}

#[test]
fn test_indented_root_after_empty_lines_is_rejected_without_mutation() {
    let dir = tempdir().unwrap();
    let result = create_tree("\n  \n  project/\n    src/", dir.path(), false, false);

    assert_eq!(
        result.unwrap_err().to_string(),
        "Root directory (line 3) should not be indented"
    );
    assert!(!dir.path().join("project").exists());
}

#[test]
fn test_creating_in_existing_directory() -> io::Result<()> {
    let dir = tempdir()?;

    // Create the root directory first
    let project_dir = dir.path().join("project");
    fs::create_dir(&project_dir)?;

    let input = "\
project/
└── src/
    └── main.rs";

    create_tree(input, dir.path(), false, false)?;

    verify_structure(
        dir.path(),
        &["project/", "project/src/", "project/src/main.rs"],
    )
}

#[test]
fn test_inconsistent_indentation() {
    let dir = tempdir().unwrap();
    let input = "\
project/
  src/
   main.rs";

    let result = create_tree(input, dir.path(), false, false);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Inconsistent indentation"));
}

#[test]
fn test_tab_indentation() -> io::Result<()> {
    let dir = tempdir()?;

    let input = "\
project/
\tsrc/
\t\tmain.rs
\t\tlib.rs
\tCargo.toml";

    create_tree(input, dir.path(), false, false)?;

    verify_structure(
        dir.path(),
        &[
            "project/",
            "project/src/",
            "project/src/main.rs",
            "project/src/lib.rs",
            "project/Cargo.toml",
        ],
    )
}

#[test]
fn test_force_overwrite() -> io::Result<()> {
    let dir = tempdir()?;

    // Create an initial structure
    let initial_input = "\
project/
  src/
    main.rs";

    create_tree(initial_input, dir.path(), false, false)?;

    // Write some content to main.rs
    fs::write(dir.path().join("project/src/main.rs"), "initial content")?;

    // Try to create a different structure with the same root
    let new_input = "\
project/
  src/
    main.rs
    lib.rs";

    // First without force (should preserve main.rs content)
    create_tree(new_input, dir.path(), false, false)?;
    assert_eq!(
        fs::read_to_string(dir.path().join("project/src/main.rs"))?,
        "initial content"
    );

    // Then with force (should overwrite main.rs)
    create_tree(new_input, dir.path(), true, false)?;
    assert_eq!(
        fs::read_to_string(dir.path().join("project/src/main.rs"))?,
        ""
    );

    Ok(())
}

#[test]
fn test_force_preserves_unlisted_files_in_existing_root() -> io::Result<()> {
    let dir = tempdir()?;
    let project_dir = dir.path().join("project");
    fs::create_dir(&project_dir)?;
    fs::write(project_dir.join("notes.txt"), "keep this content")?;

    let input = "\
project/
  src/
    main.rs";

    create_tree(input, dir.path(), true, false)?;

    assert_eq!(
        fs::read_to_string(project_dir.join("notes.txt"))?,
        "keep this content"
    );
    assert!(project_dir.join("src/main.rs").is_file());

    Ok(())
}

#[test]
fn test_adjacent_empty_directories_are_siblings() -> io::Result<()> {
    let dir = tempdir()?;
    let input = "project/\n  first/\n  second/\n  README.md";

    create_tree(input, dir.path(), false, false)?;

    verify_structure(
        dir.path(),
        &["project/first/", "project/second/", "project/README.md"],
    )?;
    assert!(!dir.path().join("project/first/second").exists());
    Ok(())
}

#[test]
fn test_rejects_path_traversal() {
    let dir = tempdir().unwrap();
    let result = create_tree("project/\n  ../\n    escaped.txt", dir.path(), false, false);

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    assert!(!dir.path().join("escaped.txt").exists());
}

#[test]
fn test_rejects_children_of_files() {
    let dir = tempdir().unwrap();
    let result = create_tree(
        "project/\n  file.txt\n    child.txt",
        dir.path(),
        false,
        false,
    );

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    assert!(!dir.path().join("project").exists());
}

#[test]
fn test_rejects_non_directory_root() {
    let dir = tempdir().unwrap();
    let result = create_tree("project", dir.path(), false, false);

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidData);
    assert!(!dir.path().join("project").exists());
}

#[test]
fn test_dry_run_does_not_modify_filesystem() -> io::Result<()> {
    let dir = tempdir()?;
    create_tree("project/\n  src/\n    main.rs", dir.path(), false, true)?;

    assert!(!dir.path().join("project").exists());
    Ok(())
}

#[test]
fn test_file_directory_conflict_requires_force() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("project/entry");
    fs::create_dir_all(&path)?;

    let result = create_tree("project/\n  entry", dir.path(), false, false);

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::AlreadyExists);
    assert!(path.is_dir());
    Ok(())
}

#[test]
fn test_force_replaces_directory_with_file() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("project/entry");
    fs::create_dir_all(&path)?;
    fs::write(path.join("contents.txt"), "preserved without force")?;

    create_tree("project/\n  entry", dir.path(), true, false)?;

    assert!(path.is_file());
    Ok(())
}

#[test]
fn test_directory_file_conflict_requires_force() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("project/entry");
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, "contents")?;

    let result = create_tree("project/\n  entry/", dir.path(), false, false);

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read_to_string(path)?, "contents");
    Ok(())
}

#[test]
fn test_force_replaces_file_with_directory() -> io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("project/entry");
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, "contents")?;

    create_tree("project/\n  entry/", dir.path(), true, false)?;

    assert!(path.is_dir());
    Ok(())
}

#[cfg(unix)]
#[test]
fn test_rejects_symlinked_directory_even_with_force() -> io::Result<()> {
    use std::os::unix::fs::symlink;

    let dir = tempdir()?;
    let outside = tempdir()?;
    let project = dir.path().join("project");
    fs::create_dir(&project)?;
    symlink(outside.path(), project.join("linked"))?;

    let result = create_tree(
        "project/\n  linked/\n    escaped.txt",
        dir.path(),
        true,
        false,
    );

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    assert!(!outside.path().join("escaped.txt").exists());
    Ok(())
}

#[cfg(unix)]
#[test]
fn test_rejects_symlinked_file_even_with_force() -> io::Result<()> {
    use std::os::unix::fs::symlink;

    let dir = tempdir()?;
    let outside = tempdir()?;
    let target = outside.path().join("target.txt");
    fs::write(&target, "keep this content")?;
    let project = dir.path().join("project");
    fs::create_dir(&project)?;
    symlink(&target, project.join("linked.txt"))?;

    let result = create_tree("project/\n  linked.txt", dir.path(), true, false);

    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    assert_eq!(fs::read_to_string(target)?, "keep this content");
    Ok(())
}
