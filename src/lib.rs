#![allow(unused)]
use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash, fmt::{Display, Debug}};
use serde::{Deserialize, Serialize};
use std::fmt;

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

// SERDE INTO TRIVIAL GRAPH FORMAT
/*
1 First node
2 Second node
#
1 2 Edge between the two
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStructure {
    pub first_node: String,
    pub second_node: String,
    pub edge: String,
}
// fn node_to_string<T>(mut node: Node<T>) -> String
// where T: std::fmt::Display + std::fmt::Debug
// {
//     format!("{:?}", node)
// }

pub fn serial_triv<T>(graph: &Graph<T>) 
where T: Copy + Display + ToString + std::fmt::Debug {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("serial_graph.yml")
        .expect("Couldn't open file");

    let gr_lenght = graph.nodes.len();
    for i in 0..gr_lenght {
        format!("{:?}", graph.nodes[i]);
    }
    let node_1 = format!("{:?}", graph.nodes[0]);
    let node_2 = format!("{:?}", graph.nodes[1]);
    let edge_1 = format!("{:?}", graph.edges[0]);
    
    let a = GraphStructure {
        first_node: node_1,
        second_node: node_2,
        edge: edge_1,
    };

    let mut serialised_graph: HashMap<usize, String> = HashMap::new();

    let serialized = serde_yaml::to_string(&a)
        .unwrap()
        .clone()
        .into_bytes();
    let serde_content = serialized
        .into_iter()
        .take_while(|&x| x != 0)
        .collect::<Vec<_>>();
    let serde_data = String::from_utf8(serde_content).expect("Invalid utf8 message");
    serialised_graph.insert(9, serde_data);

    serde_yaml::to_writer(file, &serialised_graph).unwrap();
}
pub fn deserial_triv() {}

// BREADTH FIRST SEARCH
// Use a list that stores nodes that need to be browsed.
// In one iteration of the algorythm:
// - if the list is not empty, the node is extracted from the list
// - the extracted node is visited (processed)
// - all of the children are placed into the list

pub fn bfs<T>(graph: &Graph<T>, root: Node<T>, target: Node<T>) -> Option<Vec<T>> 
where T: PartialEq + Copy + Hash + Eq + Debug {
    println!("root: {:?}; target: {:?}", root, target);
    let mut visited: HashSet<Node<T>> = HashSet::new();
    let mut history: Vec<T> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);
    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        if currentnode == target {
            println!("Goal is found: {:?}", history);
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

// pub fn bfs(graph: GraphType, start_node: i32, end_node: i32) -> Option<Vec<Option<i32>>> {
//     let mut queue = Queue::new();
//     queue.enqueue(start_node);

//     let mut visisted_nodes = vec![false; graph.len()];
//     visisted_nodes[0] = true;

//     let mut prev: Vec<Option<i32>> = vec![None; graph.len()];

//     'outer: while !queue.is_empty() {
//         let current_node = queue.dequeue();
//         for v in graph[current_node as usize].iter() {
//             if *v == end_node {
//                 prev[*v as usize] = Some(current_node);
//                 break 'outer;
//             }

//             if !visisted_nodes[*v as usize] {
//                 queue.enqueue(*v);
//                 visisted_nodes[*v as usize] = true;
//                 prev[*v as usize] = Some(current_node);
//             }
//         }
//     }

//     let mut path = Vec::new();
//     let mut at = Some(end_node);
//     while at != None {
//         path.push(at);
//         at = prev[at.unwrap_or(0) as usize];
//     }

//     path.reverse();
    
//     return match path[0] {
//         Some(x) if x == start_node => Some(path),
//         _ => None,
//     };
// }

fn main() {}