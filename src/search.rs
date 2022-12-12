use std::fmt::Debug;

// Custom iterator
#[derive(Clone, Debug)]
pub struct GraphIter {
    // Indexed Nodes
    stack: Vec<u64>,
    // Indexes of visited Nodes
    visited: Vec<u64>,
}

impl GraphIter {
    pub fn new(root: Option<u64>) -> Result<Self, String> {
        // Find root if root exist
        if let Some(root) = root {
            Ok(
                GraphIter {
                    stack: vec![root],
                    visited: vec![],
                }
            )
        } else {
            Err(String::from("Please set a root Node"))
        }
    }
}