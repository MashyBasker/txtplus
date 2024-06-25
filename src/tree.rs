/*
Implementation of tree ASCII representation based on the bullet point directive

@@start::tree
- First
    - Second
    - Third
- Fourth
@@end::tree

Renders to

.
    ├── First
    │   ├── Second
    │   └── Third
    └── Fourth

 */

use std::{fs::OpenOptions, io::Write};

#[derive(Debug)]
struct TreeNode {
    value: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(value: String) -> Self {
        TreeNode {
            value,
            children: vec![],
        }
    }
}

/// Function to parse bullet points and build a tree structure
fn parse_bullet_points(lines: &[&str]) -> TreeNode {
    // Helper function to add child nodes recursively
    fn add_children(node: &mut TreeNode, lines: &[&str], level: usize) {
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let indent_level = line.chars().take_while(|&c| c == ' ').count() / 4;
            let value = line.trim_start().trim_start_matches('-').trim().to_string();

            if indent_level == level {
                let mut child_node = TreeNode::new(value);
                add_children(&mut child_node, &lines[i + 1..], level + 1);
                node.children.push(child_node);
            } else if indent_level < level {
                break; // Exit if indent level decreases
            }

            i += 1;
        }
    }

    let mut root = TreeNode::new(".".to_string()); // Start with dot for root node
    add_children(&mut root, lines, 0);
    root
}

fn generate_tree_string(node: &TreeNode, prefix: String, is_last: bool) -> String {
    let connector = if is_last { "└── " } else { "├── " };
    let mut current_prefix = prefix.clone();
    if !prefix.is_empty() {
        current_prefix.push_str(connector);
    }

    let mut tree_string = format!("{}{}\n", current_prefix, node.value.trim());

    for (i, child) in node.children.iter().enumerate() {
        let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
        tree_string.push_str(&generate_tree_string(
            child,
            new_prefix,
            i == node.children.len() - 1,
        ));
    }

    tree_string
}

pub fn render_tree(file_write_path: &String, directives: &mut Vec<String>) -> std::io::Result<()> {
    let _ = directives.pop();
    let slice_of_strs = &directives.iter().map(|s| s.as_str()).collect::<Vec<&str>>()[..];
    let tree = parse_bullet_points(slice_of_strs);
    let tree_string = generate_tree_string(&tree, "".to_string(), true);
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_write_path)?;
    f.write_all(tree_string.as_bytes())?;
    Ok(())
}
