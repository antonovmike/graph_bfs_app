#![allow(unused)]
use std::collections::HashSet;

// TEST
pub fn test(first: u8, second: u8) -> u8 {
    first + second
}

// ADD AND REMOVE NODES
pub fn add_node(graph: Graph, to_add: Node) {}
pub fn rem_node(graph: Graph, to_remove: Node) {}

// ADD AND REMOVE DIRECTED EDGES
pub fn add_edge(graph: Graph, to_add: Edge) {}
pub fn rem_edge(graph: Graph, to_remove: Edge) {}

// SERDE INTO TRIVIAL GRAPH FORMAT

// BREADTH FIRST SEARCH

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

pub fn breadth_first_search(graph: Graph, start_node: Node, end_node: Node, goal: Node) -> Vec<u32> {
    // Visited nodes?
    let mut visited_nodes: HashSet<Node> = HashSet::new();
    // visited_nodes insert?

    // Neighbors?

    // Reached goal

    // Each Node checked, goal isn't met

    vec![1, 2, 3]
}

fn main() {}