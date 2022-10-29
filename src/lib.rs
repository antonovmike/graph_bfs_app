#![allow(unused)]
use std::collections::HashSet;

// TEST
pub fn test(first: u8, second: u8) -> u8 {
    first + second
}

// ADD AND REMOVE NODES
pub fn add_node<N, E>(graph: Graph<N, E>, to_add: Node<N>) {}
pub fn rem_node<N, E>(graph: Graph<N, E>, to_remove: Node<N>) {}

// ADD AND REMOVE DIRECTED EDGES
pub fn add_edge<N, E>(graph: Graph<N, E>, to_add: Node<E>) {}
pub fn rem_edge<N, E>(graph: Graph<N, E>, to_remove: Node<E>) {}

// SERDE INTO TRIVIAL GRAPH FORMAT

// BREADTH FIRST SEARCH

// pub struct Graph<N, E> {
//     pub nodes: Node<N>,
//     pub edges: Edge<E>,
// }
// pub struct Graph<N, E> {
//     pub nodes: Vec<Node<N>>,
//     pub edges: Vec<Edge<E>>,
// }
pub struct Graph<N, E> {
    pub nodes: Vec<N>,
    pub edges: Vec<E>,
}

pub struct Node<N>(N);
pub struct Edge<E>(E, E);

impl<N, E> Graph<N, E> {
    pub fn new(nodes: Vec<N>, edges: Vec<E>) -> Self {
        Graph { nodes: nodes, edges: edges }
    }
}

pub fn breadth_first_search<N, E>(graph: Graph<N, E>, start_node: Node<N>, end_node: Node<N>, goal: Node<N>) -> Vec<u32> {
    // Visited nodes?
    let mut visited_nodes: HashSet<Node<N>> = HashSet::new();
    // visited_nodes insert?

    // Neighbors?

    // Reached goal

    // Each Node checked, goal isn't met

    vec![1, 2, 3]
}

fn main() {}