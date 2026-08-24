# tree-create

A command-line utility to create directory structures from tree-like text input.

## Installation

```bash
cargo install tree-create
```

## Usage

`tree-create` supports multiple ways to input your directory structure:

### Interactive Editor (Default)
Simply run `tree-create` with no arguments to open your default editor:

```bash
tree-create
```

This will open your system's default editor (defined by `$EDITOR`) where you can input your directory structure. Save and close the file when you're done, and it will create the structure.

### File Input
Pass a file containing your directory structure:

```bash
tree-create input.txt
```

### Inline Input
Use the `-i` or `--inline` flag to input the structure directly in the terminal:

```bash
tree-create -i
```

### Preview Changes

Use `-n` or `--dry-run` with file or inline input to validate the complete tree and preview each filesystem action without changing anything:

```bash
tree-create --dry-run input.txt
```

The preview reports whether each path would be created, preserved, overwritten, reused, or replaced. Conflicts and symbolic links are rejected before any changes are applied.

## Input Formats

`tree-create` supports two input formats:

### ASCII Tree Format
```
my-project/
├── src/
│   ├── main.rs
│   └── lib.rs
└── Cargo.toml
```

### Simple Indented Format
You can use any consistent indentation (spaces or tabs):
```
my-project/
  src/
    main.rs
    lib.rs
  Cargo.toml
```

or
```
my-project/
    src/
        main.rs
        lib.rs
    Cargo.toml
```

## Rules and Validation

- Root directory must not be indented
- Indentation must be consistent throughout the structure
- Can't skip indentation levels
- Directories must end with a forward slash (`/`)
- Empty lines are ignored

## Examples

Create a simple Rust project structure:
```
my-project/
  src/
    main.rs
    lib.rs
  Cargo.toml
```

Create a more complex web project:
```
web-app/
  src/
    components/
      ui/
        buttons/
          primary.rs
          secondary.rs
        inputs/
          text.rs
          number.rs
    pages/
      home.rs
      about.rs
  public/
    index.html
    styles.css
  Cargo.toml
```

## Handling Existing Files and Directories

By default, `tree-create` will:
- Skip creating directories and files that already exist (preserving their contents)
- Create any new directories and files in the structure
- Print messages indicating which items were skipped and which were created

To overwrite existing files and directories, use the `--force` flag:
```bash
tree-create --force input.txt
```

When using `--force`:
- Existing files will be overwritten (emptied)
- Existing directories will be preserved, but any files specified in the tree will be overwritten
- If a file exists where a directory is specified (or vice versa), it will be replaced

**Note:** Be careful with `--force` as it will overwrite files without confirmation.

`tree-create` refuses to follow symbolic links in paths named by the input, including when `--force` is used.

## Supported Editors

The following editors are explicitly supported for interactive mode:
- vim/neovim
- nano
- VS Code (automatically adds `--wait` flag)

Other editors may work but are not officially supported.

## Development

The standard Rust toolchain is sufficient:

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

The repository also provides a reproducible Nix/devenv shell and matching `just` recipes:

```bash
direnv allow       # optional automatic shell activation
nix develop        # manual shell activation
just test
just lint
```

Run `just` to list all common project commands. Pull requests run formatting, Clippy, and the complete test suite in CI.
