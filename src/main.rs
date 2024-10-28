use clap::Parser;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;
use tree_create::create_tree;

#[derive(Parser)]
#[command(
    name = "tree-create",
    about = "Creates a directory structure from a tree-like text input",
    version
)]
struct Cli {
    /// Input file containing the tree structure
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Open an interactive editor (optional, as this is the default behavior)
    #[arg(short, long)]
    edit: bool,

    /// Accept inline input
    #[arg(short, long)]
    inline: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let current_dir = env::current_dir()?;

    let input_content = if cli.inline {
        get_inline_input()?
    } else if let Some(path) = cli.input {
        fs::read_to_string(path)?
    } else {
        match get_input_from_editor() {
            Ok(content) => content,
            Err(e) if e.kind() == io::ErrorKind::Other && e.to_string() == "Editor aborted" => {
                std::process::exit(0);
            }
            Err(e) => return Err(e),
        }
    };

    create_tree(&input_content, &current_dir)
}

fn get_editor_command() -> (String, Vec<String>) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let mut parts: Vec<String> = editor.split_whitespace().map(String::from).collect();
    
    // Handle special cases for known editors
    match parts.first().map(|s| s.as_str()) {
        Some("code") => {
            if !parts.contains(&"--wait".to_string()) {
                parts.push("--wait".to_string());
            }
        },
        Some("vim") | Some("nvim") | Some("nano") => {
            // These editors work well by default
        },
        _ => {
            eprintln!("Note: Using unsupported editor '{}'. For best experience, use vim, nvim, nano, or VS Code.", parts[0]);
        }
    }
    
    let program = parts.remove(0);
    (program, parts)
}

fn get_input_from_editor() -> io::Result<String> {
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_path_buf();
    
    let template = "# Enter your tree structure below using either format:\n\
                   #\n\
                   # Tree format with ASCII characters:\n\
                   # my-project/\n\
                   # ├── src/\n\
                   # │   ├── main.rs\n\
                   # │   └── lib.rs\n\
                   # └── Cargo.toml\n\
                   #\n\
                   # Or simple indented format:\n\
                   # my-project/\n\
                   #   src/\n\
                   #     main.rs\n\
                   #     lib.rs\n\
                   #   Cargo.toml\n";
    
    let initial_content = template.to_string();
    fs::write(&temp_path, &initial_content)?;

    let (program, args) = get_editor_command();

    let status = Command::new(&program)
        .args(&args)
        .arg(&temp_path)
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Editor aborted",
        ));
    }

    let final_content = fs::read_to_string(&temp_path)?;

    // Debug print
    eprintln!("Raw content from editor:");
    eprintln!("---BEGIN---");
    eprintln!("{}", final_content);
    eprintln!("---END---");

    if final_content == initial_content {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Editor aborted",
        ));
    }

    let processed_content: String = final_content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let processed_content = processed_content.trim();

    if processed_content.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No input provided",
        ));
    }

    Ok(processed_content.to_string())
}

fn get_inline_input() -> io::Result<String> {
    println!("Enter your tree structure (press Ctrl+D when finished):");
    println!("You can use either format:");
    println!("\nTree format with ASCII characters:");
    println!("my-project/");
    println!("├── src/");
    println!("│   ├── main.rs");
    println!("│   └── lib.rs");
    println!("└── Cargo.toml");
    println!("\nOr simple indented format:");
    println!("my-project/");
    println!("  src/");
    println!("    main.rs");
    println!("    lib.rs");
    println!("  Cargo.toml");
    println!("\n--- Start typing below ---");

    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    let content = content.trim();

    if content.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No input provided",
        ));
    }

    Ok(content.to_string())
}
