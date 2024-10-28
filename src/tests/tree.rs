#[cfg(test)]
mod tree_structure_tests {
    use crate::TreeNode;
    use crate::TreeStructure;
    use std::io;

    #[test]
    fn test_parse_empty_input() {
        let result = TreeStructure::from_string("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Input is empty");
    }

    #[test]
    fn test_parse_simple_indented() -> io::Result<()> {
        let input = "\
my-project/
  src/
    main.rs
    lib.rs
  Cargo.toml";

        let tree = TreeStructure::from_string(input)?;

        assert_eq!(
            tree.nodes[0],
            TreeNode {
                name: "my-project/".to_string(),
                indent_level: 0
            }
        );
        assert_eq!(
            tree.nodes[1],
            TreeNode {
                name: "src/".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(
            tree.nodes[2],
            TreeNode {
                name: "main.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[3],
            TreeNode {
                name: "lib.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[4],
            TreeNode {
                name: "Cargo.toml".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(tree.indent_width, 2);

        Ok(())
    }

    #[test]
    fn test_parse_ascii_tree() -> io::Result<()> {
        let input = "\
my-project/
├── src/
│   ├── main.rs
│   └── lib.rs
└── Cargo.toml";

        let tree = TreeStructure::from_string(input)?;

        assert_eq!(
            tree.nodes[0],
            TreeNode {
                name: "my-project/".to_string(),
                indent_level: 0
            }
        );
        assert_eq!(
            tree.nodes[1],
            TreeNode {
                name: "src/".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(
            tree.nodes[2],
            TreeNode {
                name: "main.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[3],
            TreeNode {
                name: "lib.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[4],
            TreeNode {
                name: "Cargo.toml".to_string(),
                indent_level: 1
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_single_space_indent() -> io::Result<()> {
        let input = "\
my-project/
 src/
  main.rs
  lib.rs
 Cargo.toml";

        let tree = TreeStructure::from_string(input)?;

        assert_eq!(
            tree.nodes[0],
            TreeNode {
                name: "my-project/".to_string(),
                indent_level: 0
            }
        );
        assert_eq!(
            tree.nodes[1],
            TreeNode {
                name: "src/".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(
            tree.nodes[2],
            TreeNode {
                name: "main.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[3],
            TreeNode {
                name: "lib.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[4],
            TreeNode {
                name: "Cargo.toml".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(tree.indent_width, 1);

        Ok(())
    }

    #[test]
    fn test_parse_tab_indent() -> io::Result<()> {
        let input = "\
my-project/
\tsrc/
\t\tmain.rs
\t\tlib.rs
\tCargo.toml";

        let tree = TreeStructure::from_string(input)?;

        assert_eq!(
            tree.nodes[0],
            TreeNode {
                name: "my-project/".to_string(),
                indent_level: 0
            }
        );
        assert_eq!(
            tree.nodes[1],
            TreeNode {
                name: "src/".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(
            tree.nodes[2],
            TreeNode {
                name: "main.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[3],
            TreeNode {
                name: "lib.rs".to_string(),
                indent_level: 2
            }
        );
        assert_eq!(
            tree.nodes[4],
            TreeNode {
                name: "Cargo.toml".to_string(),
                indent_level: 1
            }
        );
        assert_eq!(tree.indent_width, 1);

        Ok(())
    }

#[test]
    fn test_indented_root_error() {
        let input = "  my-project/\n  src/";

        let result = TreeStructure::from_string(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Root directory (line 1) should not be indented"
        );
    }

    #[test]
    fn test_inconsistent_indent_error() {
        let input = "\
my-project/
  src/
   main.rs";

        let result = TreeStructure::from_string(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Inconsistent indentation at line 3. Expected a multiple of 2 spaces (found 3 spaces)"
        );
    }

    #[test]
    fn test_skipped_level_error() {
        let input = "\
my-project/
  src/
      main.rs";

        let result = TreeStructure::from_string(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid indentation at line 3. Indentation can only increase by one level at a time"
        );
    }

    #[test]
    fn test_empty_lines_are_ignored() -> io::Result<()> {
        let input = "\
my-project/

  src/
    main.rs

    lib.rs
  Cargo.toml

";

        let tree = TreeStructure::from_string(input)?;
        assert_eq!(tree.nodes.len(), 5);
        Ok(())
    }

    #[test]
    fn test_deep_nesting() -> io::Result<()> {
        let input = "\
my-project/
  src/
    components/
      ui/
        buttons/
          primary.rs
          secondary.rs
        inputs/
          text.rs
          number.rs";

        let tree = TreeStructure::from_string(input)?;
        assert_eq!(tree.nodes[0].indent_level, 0);
        assert_eq!(tree.nodes[1].indent_level, 1);
        assert_eq!(tree.nodes[2].indent_level, 2);
        assert_eq!(tree.nodes[3].indent_level, 3);
        assert_eq!(tree.nodes[4].indent_level, 4);
        assert_eq!(tree.nodes[5].indent_level, 5);
        assert_eq!(tree.nodes[6].indent_level, 5);
        assert_eq!(tree.nodes[7].indent_level, 4);
        assert_eq!(tree.nodes[8].indent_level, 5);
        assert_eq!(tree.nodes[9].indent_level, 5);

        Ok(())
    }
}
