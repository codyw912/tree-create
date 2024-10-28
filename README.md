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

## Supported Editors

The following editors are explicitly supported for interactive mode:
- vim/neovim
- nano
- VS Code (automatically adds `--wait` flag)

Other editors may work but are not officially supported.
