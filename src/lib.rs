#![allow(unused)]
use std::collections::HashSet;
use std::cmp::Eq;
use std::ops::Index;

// ADD AND REMOVE NODES
pub fn add_node<N, E>(graph: Graph<N, E>, to_add: N) -> Graph<N, E> {
    let mut new_vec = graph;
    new_vec.nodes.push(to_add);
    new_vec
}
pub fn rem_node<N, E>(graph: Graph<N, E>, to_remove: N) -> Graph<N, E> 
where
    N: PartialEq
{
    // let mut old_vec = graph;
    let mut nodes = graph.nodes;
    // if let Some(index) = nodes.iter().position(|value| *value == to_remove) {
    //     nodes.swap_remove(index);
    // }
    nodes.retain(|value| *value != to_remove);
    // new_vec
    let new_vec = Graph {
        nodes: nodes,
        edges: graph.edges,
    };
    new_vec
}

// ADD AND REMOVE DIRECTED EDGES
pub fn add_edge<N, E>(graph: Graph<N, E>, to_add: Node<E>) {}
pub fn rem_edge<N, E>(graph: Graph<N, E>, to_remove: Node<E>) {}

// SERDE INTO TRIVIAL GRAPH FORMAT

// BREADTH FIRST SEARCH
#[derive(Clone)]
pub struct Graph<N, E> {
    pub nodes: Vec<N>,
    pub edges: Vec<E>,
}

#[derive(PartialEq)]
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