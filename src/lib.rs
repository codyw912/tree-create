use std::fs;
use std::io::{self, BufRead};
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
            let indent_level = if prefixes == 0 { 0 } else { (prefixes + 3) / 4 };

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

        Ok(Self {
            nodes,
            indent_width: 2, // Default indent width for output
        })
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

        Ok(Self {
            nodes,
            indent_width: indent_width.unwrap_or(2),
        })
    }

    /// Convert the internal representation to ASCII tree format
    fn to_ascii_tree(&self) -> String {
        let mut result = Vec::new();

        // First pass: determine which levels have subsequent siblings
        let mut level_has_next = [false; 32]; // 32 levels should be enough

        for (i, node) in self.nodes.iter().enumerate() {
            let current_level = node.indent_level;

            // Look ahead to see if there are any more nodes at this level
            if let Some(next_node) = self.nodes.get(i + 1) {
                if next_node.indent_level == current_level {
                    level_has_next[current_level] = true;
                }
            }
        }

        // Second pass: generate the tree
        for node in self.nodes.iter() {
            let mut prefix = String::new();

            // Add vertical lines for previous levels that have subsequent nodes
            prefix.extend(
                level_has_next
                    .iter()
                    .take(node.indent_level)
                    .skip(1)
                    .map(|&has_next| if has_next { "│   " } else { "    " }),
            );

            // Add the appropriate branch character
            if node.indent_level > 0 {
                prefix.push_str(if level_has_next[node.indent_level] {
                    "├── "
                } else {
                    "└── "
                });
            }

            result.push(format!("{}{}", prefix, node.name));
        }

        result.join("\n")
    }
}

/// Check if input is using ASCII tree format
fn is_ascii_tree(input: &str) -> bool {
    input.contains("├──") || input.contains("└──") || input.contains("│")
}

pub fn create_tree(input: &str, base_path: &Path, force: bool, dry_run: bool) -> io::Result<()> {
    // Convert the input to our internal representation
    let tree = TreeStructure::from_string(input)?;

    // Then convert to ASCII tree format for processing
    let ascii_tree = tree.to_ascii_tree();

    let reader = io::BufReader::new(ascii_tree.as_bytes());
    let mut lines = reader.lines().peekable();

    // Get the root directory name from the first line
    let root_name = if let Some(Ok(first_line)) = lines.next() {
        first_line.trim_end_matches('/').to_string()
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Input is empty"));
    };

    let base_path = base_path.join(&root_name);
    if dry_run {
        println!("dry-run Would create directory: {}", base_path.display());
    } else if base_path.exists() {
        if base_path.is_file() {
            if force {
                fs::remove_file(&base_path)?;
                fs::create_dir_all(&base_path)?;
                println!("Overwrote file with root directory: {:?}", base_path);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "A file exists where the root directory is required: {}",
                        base_path.display()
                    ),
                ));
            }
        } else {
            println!("Using existing root directory: {:?}", base_path);
        }
    } else {
        fs::create_dir_all(&base_path)?;
        println!("Created root directory: {:?}", base_path);
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
            if dry_run {
                println!("dry-run Would create directory: {}", full_path.display());
            } else if full_path.exists() {
                if force {
                    if full_path.is_file() {
                        fs::remove_file(&full_path)?;
                        fs::create_dir_all(&full_path)?;
                        println!("Overwrote file with directory: {:?}", full_path);
                    } else {
                        println!("Using existing directory: {:?}", full_path);
                    }
                } else {
                    println!("Directory already exists: {:?}", full_path);
                }
            } else {
                fs::create_dir_all(&full_path)?;
                println!("Created directory: {:?}", full_path);
            }
            path_stack.push(full_path);
        } else {
            if dry_run {
                println!("dry-run Would create file: {}", full_path.display());
            } else {
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                if full_path.exists() {
                    if force {
                        if full_path.is_dir() {
                            fs::remove_dir_all(&full_path)?;
                        }
                        fs::write(&full_path, "")?;
                        println!("Overwrote existing file: {:?}", full_path);
                    } else {
                        println!("File already exists: {:?}", full_path);
                    }
                } else {
                    fs::File::create(&full_path)?;
                    println!("Created file: {:?}", full_path);
                }
            }
        }
    }

    Ok(())
}
