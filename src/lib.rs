#![allow(unused)]
use std::collections::HashSet;
use std::collections::VecDeque;
use core::hash::Hash;
// use std::hash::Hash;

// --> ADD AND REMOVE NODES

pub fn add_node<N, E>(graph: Graph<N, E>, to_add: N) -> Graph<N, E> {
    let mut new_vec = graph;
    new_vec.nodes.push(to_add);
    new_vec
}

pub fn rem_node<N, E>(graph: Graph<N, E>, to_remove: N) -> Graph<N, E> 
where
    N: PartialEq
{
    let mut nodes = graph.nodes;
    nodes.retain(|value| *value != to_remove);
    let new_vec = Graph {
        nodes: nodes,
        edges: graph.edges,
    };
    new_vec
}

// --> ADD AND REMOVE (not)DIRECTED EDGES

pub fn add_edge<N, E>(graph: Graph<N, E>, to_add: E) -> Graph<N, E> {
    let mut new_vec = graph;
    new_vec.edges.push(to_add);
    new_vec
}

pub fn rem_edge<N, E>(graph: Graph<N, E>, to_remove: E) -> Graph<N, E> 
where
    E: PartialEq
{
    let mut edges = graph.edges;
    edges.retain(|value| *value != to_remove);
    let new_vec = Graph {
        nodes: graph.nodes,
        edges: edges,
    };
    new_vec
}

// SERDE INTO TRIVIAL GRAPH FORMAT

// BREADTH FIRST SEARCH
#[derive(Clone)]
pub struct Graph<N, E> {
    pub nodes: Vec<N>,
    pub edges: Vec<E>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Node<N>(N);
#[derive(Clone)]
pub struct Edge<E>(E, E);

impl<N, E> Graph<N, E> {
    pub fn new(nodes: Vec<N>, edges: Vec<E>) -> Self {
        Graph { nodes: nodes, edges: edges }
    }
}

pub fn breadth_first_search<N, E>(
    graph: Graph<N, E>, start_node: N, end_node: N, goal: N
) -> String 
where N: Eq, N: Hash, N: PartialEq
{
    // Visited nodes?
    let mut visited_nodes: HashSet<N> = HashSet::new();
    let mut visited_nodes: Vec<bool> = vec![false; graph.nodes.len()];
    visited_nodes[0] = true;
    // visited_nodes insert?
    // let mut history: Vec<Node<N>> = Vec::new();
    // Sequence?
    // let mut queue: VecDeque<N> = VecDeque::new();
    // queue.push_back(start_node);

    // Neighbors?
    // let mut previous: Vec<Option<N>> = vec![None; graph.nodes.len()];

    // Reached goal

    // Each Node checked, goal isn't met

    "a".to_string()
}

fn main() {}
