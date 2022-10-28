// BREADTH FIRST SEARCH

use std::collections::HashSet;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct Node(u32);
pub struct Edge(u32, u32);

impl Graph {
    pub fn new(node: Vec<Node>, edge: Vec<Edge>) -> Self {
        Graph { nodes: node, edges: edge }
    }
}

pub fn breadth_first_search(graph: Graph, start_point: Node, end_point: Node, goal: Node) {
    // Visited nodes?
    let mut visited_nodes: HashSet<Node> = HashSet::new();
    // visited_nodes insert?

    // Neighbors?

    // Reached goal

    // Each Node checked, goal isn't met
}

fn main() {}