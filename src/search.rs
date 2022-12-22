use super::Graph;
use std::fmt::{Display, Debug};

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

    // Breadth-first search
    pub fn bfs<N>(&mut self, graph: &Graph<N>) -> Option<u64> where N: Copy + Debug + std::cmp::PartialEq {
        while !self.stack.is_empty() {
            // Get next index
            let node_index = self.stack.remove(0);

            // Process visited nodes
            if self.visited.contains(&node_index) { continue; }
            self.visited.push(node_index);

            if let node = graph.get_node(&node_index) {
                // Check neighbours ...
                
                return Some(node_index)
            } else {
                format!("Node {} does not exist", node_index);
            }
        }
        None
    }
}

// Ok here is another problem
fn connected() -> Vec<usize> { vec![0] }