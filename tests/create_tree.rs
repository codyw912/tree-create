use std::fs;
use std::io;
use tempfile::tempdir;
use tree_create::create_tree;

/// Helper function to verify directory structure
fn verify_structure(dir: &std::path::Path, expected_paths: &[&str]) -> io::Result<()> {
    for path in expected_paths {
        let full_path = dir.join(path);
        assert!(
            full_path.exists(),
            "Path not found: {:?}",
            full_path
        );
        
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

    create_tree(input, dir.path(), false)?; 

    verify_structure(dir.path(), &[
        "project/",
        "project/src/",
        "project/src/main.rs",
        "project/src/lib.rs",
        "project/Cargo.toml",
    ])
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

    create_tree(input, dir.path(), false)?;

    verify_structure(dir.path(), &[
        "project/",
        "project/src/",
        "project/src/main.rs",
        "project/src/lib.rs",
        "project/Cargo.toml",
    ])
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

    create_tree(input, dir.path(), false)?;

    verify_structure(dir.path(), &[
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
    ])
}

#[test]
fn test_empty_input() {
    let dir = tempdir().unwrap();
    let result = create_tree("", dir.path(), false);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Input is empty"
    );
}

#[test]
fn test_indented_root_error() {
    let dir = tempdir().unwrap();
    let input = "  project/\n  src/";
    
    let result = create_tree(input, dir.path(), false);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Root directory (line 1) should not be indented"
    );
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

    create_tree(input, dir.path(), false)?;

    verify_structure(dir.path(), &[
        "project/",
        "project/src/",
        "project/src/main.rs",
    ])
}

#[test]
fn test_inconsistent_indentation() {
    let dir = tempdir().unwrap();
    let input = "\
project/
  src/
   main.rs";
    
    let result = create_tree(input, dir.path(), false);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Inconsistent indentation"));
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

    create_tree(input, dir.path(), false)?;

    verify_structure(dir.path(), &[
        "project/",
        "project/src/",
        "project/src/main.rs",
        "project/src/lib.rs",
        "project/Cargo.toml",
    ])
}

#[test]
fn test_force_overwrite() -> io::Result<()> {
    let dir = tempdir()?;
    
    // Create an initial structure
    let initial_input = "\
project/
  src/
    main.rs";

    create_tree(initial_input, dir.path(), false)?;
    
    // Write some content to main.rs
    fs::write(
        dir.path().join("project/src/main.rs"),
        "initial content"
    )?;
    
    // Try to create a different structure with the same root
    let new_input = "\
project/
  src/
    main.rs
    lib.rs";

    // First without force (should preserve main.rs content)
    create_tree(new_input, dir.path(), false)?;
    assert_eq!(
        fs::read_to_string(dir.path().join("project/src/main.rs"))?,
        "initial content"
    );
    
    // Then with force (should overwrite main.rs)
    create_tree(new_input, dir.path(), true)?;
    assert_eq!(
        fs::read_to_string(dir.path().join("project/src/main.rs"))?,
        ""
    );
    
    Ok(())
}
