use std::fs;
use std::io;
use std::path::Path;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct TreeNode {
    name: String,
    indent_level: usize,
}

#[derive(Debug)]
struct TreeStructure {
    nodes: Vec<TreeNode>,
    #[allow(dead_code)] // This is used implicitly in Debug
    indent_width: usize,
}

impl TreeStructure {
    /// Parse a string in either ASCII tree format or indented format into our internal representation
    pub fn from_string(input: &str) -> io::Result<Self> {
        if input.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Input is empty"));
        }

        let first_line = input.lines().next().unwrap_or("");
        let leading_spaces = first_line.chars().take_while(|c| c.is_whitespace()).count();

        if leading_spaces > 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Root directory (line 1) should not be indented",
            ));
        }

        if is_ascii_tree(input) {
            Self::from_ascii_tree(input)
        } else {
            Self::from_indented(input)
        }
    }

    /// Convert ASCII tree format ("├── file.txt") to our internal representation
    fn from_ascii_tree(input: &str) -> io::Result<Self> {
        let mut nodes: Vec<TreeNode> = Vec::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Count the indent level based on the tree characters
            let prefixes = line
                .chars()
                .take_while(|&c| c == ' ' || c == '│' || c == '├' || c == '└')
                .count();

            // Each level consists of either "│   " (4 chars) or "├── " (4 chars)
            let indent_level = if prefixes == 0 {
                0
            } else {
                prefixes.div_ceil(4)
            };

            // Extract the name by trimming tree characters
            let name = line
                .trim_start_matches(|c: char| {
                    c.is_whitespace() || c == '│' || c == '├' || c == '└' || c == '─'
                })
                .to_string();

            nodes.push(TreeNode { name, indent_level });
        }

        if nodes.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No valid nodes found",
            ));
        }

        let tree = Self {
            nodes,
            indent_width: 2, // Default indent width for output
        };
        tree.validate()?;
        Ok(tree)
    }

    /// Parse simple indented format into our internal representation
    fn from_indented(input: &str) -> io::Result<Self> {
        let mut nodes: Vec<TreeNode> = Vec::new();
        let mut indent_width = None;

        for (line_num, line) in input.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let spaces = line.chars().take_while(|c| c.is_whitespace()).count();
            let name = line.trim().to_string();

            // If this is the first indented line, use it to determine indent width
            if spaces > 0 && indent_width.is_none() {
                indent_width = Some(spaces);
            }

            let indent_width = indent_width.unwrap_or(2);

            // Validate indentation is consistent
            if spaces % indent_width != 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Inconsistent indentation at line {}. Expected a multiple of {} spaces (found {} spaces)",
                        line_num + 1, indent_width, spaces
                    )
                ));
            }

            let indent_level = spaces / indent_width;

            // Validate indent level doesn't skip levels
            if let Some(prev_node) = nodes.last() {
                if indent_level > prev_node.indent_level + 1 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Invalid indentation at line {}. Indentation can only increase by one level at a time",
                            line_num + 1
                        )
                    ));
                }
            }

            nodes.push(TreeNode { name, indent_level });
        }

        if nodes.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No valid nodes found",
            ));
        }

        let tree = Self {
            nodes,
            indent_width: indent_width.unwrap_or(2),
        };
        tree.validate()?;
        Ok(tree)
    }

    fn validate(&self) -> io::Result<()> {
        let root = &self.nodes[0];
        if !root.name.ends_with('/') {
            return Err(invalid_data(
                "Root node must be a directory ending with '/'",
            ));
        }

        for (index, node) in self.nodes.iter().enumerate() {
            validate_node_name(&node.name, index + 1)?;

            if index > 0 && node.indent_level == 0 {
                return Err(invalid_data(format!(
                    "Multiple root nodes are not supported (line {})",
                    index + 1
                )));
            }

            if let Some(previous) = index.checked_sub(1).and_then(|i| self.nodes.get(i)) {
                if node.indent_level > previous.indent_level + 1 {
                    return Err(invalid_data(format!(
                        "Invalid indentation at line {}. Indentation can only increase by one level at a time",
                        index + 1
                    )));
                }
                if node.indent_level > previous.indent_level && !previous.name.ends_with('/') {
                    return Err(invalid_data(format!(
                        "File '{}' cannot contain children (line {})",
                        previous.name,
                        index + 1
                    )));
                }
            }
        }

        Ok(())
    }
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn validate_node_name(name: &str, line: usize) -> io::Result<()> {
    let path_name = name.strip_suffix('/').unwrap_or(name);
    if path_name.is_empty()
        || path_name == "."
        || path_name == ".."
        || path_name.contains('/')
        || path_name.contains('\\')
    {
        return Err(invalid_data(format!(
            "Invalid path name '{}' at line {}: names must be a single safe path component",
            name, line
        )));
    }
    Ok(())
}

/// Check if input is using ASCII tree format
fn is_ascii_tree(input: &str) -> bool {
    input.contains("├──") || input.contains("└──") || input.contains("│")
}

pub fn create_tree(input: &str, base_path: &Path, force: bool, dry_run: bool) -> io::Result<()> {
    let tree = TreeStructure::from_string(input)?;
    let root = &tree.nodes[0];
    let root_path = base_path.join(root.name.trim_end_matches('/'));
    create_directory(&root_path, force, dry_run, true)?;

    // parents[level] is the directory that contains nodes at that indentation level.
    let mut parents = vec![root_path];
    for node in tree.nodes.iter().skip(1) {
        let parent = parents.get(node.indent_level - 1).ok_or_else(|| {
            invalid_data(format!("No parent directory exists for '{}'", node.name))
        })?;
        let name = node.name.strip_suffix('/').unwrap_or(&node.name);
        let full_path = parent.join(name);

        if node.name.ends_with('/') {
            create_directory(&full_path, force, dry_run, false)?;
            parents.truncate(node.indent_level);
            parents.push(full_path);
        } else {
            create_file(&full_path, force, dry_run)?;
        }
    }

    Ok(())
}

fn create_directory(path: &Path, force: bool, dry_run: bool, root: bool) -> io::Result<()> {
    if dry_run {
        println!("dry-run Would create directory: {}", path.display());
    } else if path.is_file() {
        if !force {
            let message = if root {
                format!(
                    "A file exists where the root directory is required: {}",
                    path.display()
                )
            } else {
                format!(
                    "A file exists where a directory is required: {}",
                    path.display()
                )
            };
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, message));
        }
        fs::remove_file(path)?;
        fs::create_dir_all(path)?;
        println!("Overwrote file with directory: {:?}", path);
    } else if path.is_dir() {
        println!("Using existing directory: {:?}", path);
    } else {
        fs::create_dir_all(path)?;
        println!("Created directory: {:?}", path);
    }
    Ok(())
}

fn create_file(path: &Path, force: bool, dry_run: bool) -> io::Result<()> {
    if dry_run {
        println!("dry-run Would create file: {}", path.display());
    } else if path.exists() {
        if force {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            }
            fs::write(path, "")?;
            println!("Overwrote existing file: {:?}", path);
        } else {
            println!("File already exists: {:?}", path);
        }
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(path)?;
        println!("Created file: {:?}", path);
    }
    Ok(())
}
