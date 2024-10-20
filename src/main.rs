use std::env;
use std::fs;
use tree_create::create_tree;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file)?;
    let current_dir = env::current_dir()?;

    create_tree(&input, &current_dir)
}
