use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn create_tree(input: &str, base_path: &Path) -> io::Result<()> {
    let reader = io::BufReader::new(input.as_bytes());
    let mut lines = reader.lines().peekable();

    // Get the root directory name from the first line
    let root_name = if let Some(Ok(first_line)) = lines.next() {
        first_line.trim_end_matches('/').to_string()
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Input is empty"));
    };

    let base_path = base_path.join(&root_name);
    if !base_path.exists() {
        fs::create_dir_all(&base_path)?;
        println!("Created root directory: {:?}", base_path);
    } else {
        println!("Root directory already exists: {:?}", base_path);
    }

    let mut current_depth = 0;
    let mut path_stack = vec![base_path.clone()];

    for line in lines {
        let line = line?;
        let depth = line
            .chars()
            .take_while(|&c| c == ' ' || c == '│' || c == '└' || c == '├')
            .count()
            / 4;
        let name = line
            .trim_start_matches(|c: char| {
                c.is_whitespace() || c == '│' || c == '└' || c == '├' || c == '─'
            })
            .to_string();

        // Adjust the path stack based on the new depth
        while depth < current_depth && !path_stack.is_empty() {
            path_stack.pop();
            current_depth -= 1;
        }
        current_depth = depth;

        let mut full_path = path_stack
            .last()
            .cloned()
            .unwrap_or_else(|| base_path.clone());
        full_path.push(&name);

        if name.ends_with('/') {
            if !full_path.exists() {
                fs::create_dir_all(&full_path)?;
                println!("Created directory: {:?}", full_path);
            } else {
                println!("Directory already exists: {:?}", full_path);
            }
            path_stack.push(full_path);
        } else {
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            if !full_path.exists() {
                fs::File::create(&full_path)?;
                println!("Created file: {:?}", full_path);
            } else {
                println!("File already exists: {:?}", full_path);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_simple_structure() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let input = "
test_project/
    src/
        main.rs
    Cargo.toml
";
        create_tree(input, temp_dir.path())?;

        assert!(temp_dir.path().join("test_project").is_dir());
        assert!(temp_dir.path().join("test_project/src").is_dir());
        assert!(temp_dir.path().join("test_project/src/main.rs").is_file());
        assert!(temp_dir.path().join("test_project/Cargo.toml").is_file());

        Ok(())
    }

    #[test]
    fn test_create_nested_structure() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let input = "
nested_project/
    src/
        module1/
            mod.rs
        module2/
            submodule/
                mod.rs
    tests/
        integration_tests.rs
    Cargo.toml
";
        create_tree(input, temp_dir.path())?;

        assert!(temp_dir.path().join("nested_project").is_dir());
        assert!(temp_dir.path().join("nested_project/src").is_dir());
        assert!(temp_dir.path().join("nested_project/src/module1").is_dir());
        assert!(temp_dir
            .path()
            .join("nested_project/src/module1/mod.rs")
            .is_file());
        assert!(temp_dir
            .path()
            .join("nested_project/src/module2/submodule")
            .is_dir());
        assert!(temp_dir
            .path()
            .join("nested_project/src/module2/submodule/mod.rs")
            .is_file());
        assert!(temp_dir.path().join("nested_project/tests").is_dir());
        assert!(temp_dir
            .path()
            .join("nested_project/tests/integration_tests.rs")
            .is_file());
        assert!(temp_dir.path().join("nested_project/Cargo.toml").is_file());

        Ok(())
    }

    #[test]
    fn test_create_existing_structure() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        fs::create_dir(temp_dir.path().join("existing_project"))?;
        fs::create_dir(temp_dir.path().join("existing_project/src"))?;
        fs::File::create(temp_dir.path().join("existing_project/src/main.rs"))?;

        let input = "
existing_project/
    src/
        main.rs
    Cargo.toml
";
        create_tree(input, temp_dir.path())?;

        assert!(temp_dir.path().join("existing_project").is_dir());
        assert!(temp_dir.path().join("existing_project/src").is_dir());
        assert!(temp_dir
            .path()
            .join("existing_project/src/main.rs")
            .is_file());
        assert!(temp_dir
            .path()
            .join("existing_project/Cargo.toml")
            .is_file());

        Ok(())
    }
}
