# tree-create

   `tree-create` is a command-line utility that creates a directory structure based on a text input representing a tree-like structure.

   ## Usage

   ```
   tree-create <input_file>
   ```

   Where `<input_file>` is a text file containing the desired directory structure in a tree-like format.

   ## Example Input File

   ```
   my_project/
       src/
           main.rs
       tests/
           test_main.rs
       Cargo.toml
   ```

   ## Building

   To build the project, ensure you have Rust installed and run:

   ```
   cargo build --release
   ```

   ## Running Tests

   To run the tests:

   ```
   cargo test
   ```

   ## License

   This project is licensed under the MIT License - see the LICENSE file for details.
