#![allow(unused)]
use std::{collections::{HashSet, VecDeque}, hash::Hash};

#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<Edge>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node<T> (pub T);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Edge(pub i32, pub i32);

impl<T> Graph<T> {
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }
}

impl<T> From<T> for Node<T> {
    fn from(item: T) -> Self {
        Node(item)
    }
}

impl<T> Node<T> {
    pub fn value(&self) -> T 
    where T: Copy{
        self.0
    }

    pub fn neighbors(&self, graph: &Graph<T>) -> Vec<Node<T>> 
    where T: PartialEq + Copy + Hash
    {
        graph
            .nodes
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.0.into())
            .collect()
    }
}

// --> ADD AND REMOVE NODES

pub fn add_node<T>(graph: Graph<T>, to_add: Node<T>) -> Graph<T> {
    let mut new_vec = graph;
    new_vec.nodes.push(to_add);
    new_vec
}

pub fn rem_node<T>(graph: Graph<T>, to_remove: Node<T>) -> Graph<T> 
where T: PartialEq
{
    let mut nodes = graph.nodes;
    nodes.retain(|value: &Node<T> | *value != to_remove);
    let new_vec: Graph<T> = Graph {
        nodes: nodes,
        edges: graph.edges,
    };
    new_vec
}

// --> ADD AND REMOVE EDGES

pub fn add_edge<T>(graph: Graph<T>, to_add: Edge) -> Graph<T> {
    let mut new_vec = graph;
    new_vec.edges.push(to_add);
    new_vec
}

pub fn rem_edge<T>(graph: Graph<T>, to_remove: Edge) -> Graph<T> {
    let mut edges: Vec<Edge> = graph.edges;
    edges.retain(|value: &Edge | *value != to_remove);
    let new_vec = Graph {
        nodes: graph.nodes,
        edges: edges,
    };
    new_vec
}

// --> SERDE INTO TRIVIAL GRAPH FORMAT
// ...

// --> BREADTH FIRST SEARCH

pub fn bfs<T>(graph: &Graph<T>, root: Node<T>, target: Node<T>) -> Option<Vec<T>> 
where T: PartialEq + Copy + Hash + Eq {
    // println!("root {:?} target {:?}", root, target);
    let mut visited: HashSet<Node<T>> = HashSet::new();
    let mut history: Vec<T> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);
    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        if currentnode == target {
            // println!("Goal is found: {:?}", history);
            return Some(history);
        }

        for neighbor in currentnode.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn main() {}