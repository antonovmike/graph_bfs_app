#![allow(unused)]
use std::{
    collections::{BTreeMap, HashSet, VecDeque}, 
    hash::Hash, fmt::{Display, Debug}, io::BufRead
};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<Edge<T>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct Node<T> (pub T);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub struct Edge<T>(pub Node<T>, pub Node<T>);

impl<T> Graph<T> {
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge<T>>) -> Self {
        Graph { nodes, edges }
    }
}

impl<T> From<T> for Node<T> {
    fn from(item: T) -> Self {
        Node(item)
    }
}

impl<T> Node<T> {
    pub fn value(&self) -> Node<T> 
    where T: Copy {
        Node(self.0)
    }

    pub fn neighbors(&self, graph: &Graph<T>) -> Vec<Node<T>> 
    where T: PartialEq + Copy + Hash {
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
where T: PartialEq {
    let mut nodes = graph.nodes;
    nodes.retain(|value: &Node<T> | *value != to_remove);
    let new_vec: Graph<T> = Graph {
        nodes: nodes,
        edges: graph.edges,
    };
    new_vec
}

// --> ADD AND REMOVE EDGES

pub fn add_edge<T>(graph: Graph<T>, to_add: Edge<T>) -> Graph<T> {
    let mut new_vec = graph;
    new_vec.edges.push(to_add);
    new_vec
}

pub fn rem_edge<T>(graph: Graph<T>, to_remove: Edge<T>) -> Graph<T> 
where T: PartialEq {
    let mut edges: Vec<Edge<T>> = graph.edges;
    edges.retain(|value: &Edge<T> | *value != to_remove);
    let new_vec = Graph {
        nodes: graph.nodes,
        edges: edges,
    };
    new_vec
}


// --> SERDE INTO TRIVIAL GRAPH FORMAT
/*
1 First node
2 Second node
#
1 2 Edge between the two
*/

#[derive(Debug, Deserialize, Serialize)]
// pub struct GraphStructure {
//     pub first_node: String,
//     pub second_node: String,
//     pub edge: String,
// }
pub struct GraphStructure<T> {
    pub first_node: Vec<Node<T>>,
    pub second_node: Vec<Node<T>>,
    pub edge: Vec<Edge<T>>,
}

impl<T> GraphStructure<T> {
    pub fn new(first_node: Vec<Node<T>>, second_node: Vec<Node<T>>, edge: Vec<Edge<T>>) -> Self {
        GraphStructure { first_node, second_node, edge }
    }
}
// pub trait Serialize<'de, T>: GraphStructure<T> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Serialize<'de>;
// }
// impl<'de, T> Serialize<'de, T> for T {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where D: Serialize<'de> {
//         deserializer.deserialize_i32(Self)
//     }
// }
fn into_structure<T>(graph: &Graph<T>, i: usize) -> GraphStructure<T>
where T: Copy + std::fmt::Display + std::fmt::Debug {
    let triivial_graph = GraphStructure {
        first_node: vec!(graph.edges[i].0),
        second_node: vec!(graph.edges[i].1),
        edge: vec!(graph.edges[i]),
    };
    triivial_graph
}

pub fn serial_triv<T>(graph: &Graph<T>) 
where T: Serialize + Copy + Display + ToString + std::fmt::Debug {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("serial_graph.yml")
        .expect("Couldn't open file");

    // 1. NAMED FIELDS
    // let mut result: BTreeMap<String, String> = BTreeMap::new();
    // let gr_lenght = graph.edges.len();
    // for i in 0..gr_lenght {
    //     let key = format!("Edge {}", i);
    //     let serialized = serde_yaml::to_string(&into_structure(graph, i))
    //         .unwrap().clone().into_bytes();
    //     let serialized: Vec<u8> = serialized
    //         .into_iter()
    //         .take_while(|&x| x != 0)
    //         .collect::<Vec<u8>>();
        
    //     let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");

    //     result.insert(key, value_serialized);
    // }

    // serde_yaml::to_writer(file, &result).unwrap();

    // 2. SIMPLE NOT NAMED FIELDS
    let triv_gr: GraphStructure<T> = GraphStructure { 
        first_node: graph.nodes.clone(),
        second_node: graph.nodes.clone(),
        edge: graph.edges.clone() 
    };
    let serialized = serde_yaml::to_string(&triv_gr).unwrap();
    println!("serialized\n{}", serialized);
    serde_yaml::to_writer(file, &serialized).unwrap();
}

// RETURNS GraphStructure
pub fn deserial_triv<'de, T>(path: &'de str) -> GraphStructure<T>
// -> Graph<T> 
where T: Deserialize<'de> + Copy + Display + ToString + std::fmt::Debug {
    // let file = std::fs::OpenOptions::new()
    //     .write(true)
    //     .open(path)
    //     .expect("Couldn't open file");
    // let mut all_lines: Vec<String> = vec![];

    // for line in std::io::BufReader::new(
    //     std::fs::File::open(path).expect("Failed at opening file.")
    // ).lines() {
    //     let words = line.unwrap();
    //     all_lines.push(words)
    // }
    
    // let mut edge_index = 0;

    // let mut vec_of_graphs: Vec<GraphStructure<T>> = vec![];
    
    // for i in 0..all_lines.len() {
    //     let edge_index_string = format!("Edge {}: |", edge_index);
    //     if all_lines[i].contains(&edge_index_string[1..]) {
    //         let each_part = format!(
    //             "{}\n{}\n{}", &all_lines[i + 1], &all_lines[i + 2], &all_lines[i + 3]
    //         );
    //         let deser: GraphStructure<T> = serde_yaml::from_str(&each_part).unwrap();
    //         vec_of_graphs.push(deser);
    //         edge_index += 1;
    //     }
    // }
    
    // to_writer(file, &result).unwrap();
    // let a = &vec_of_graphs[0].first_node.remove(5);
    // let b = *a;
    
    // vec_of_graphs

    let deserialized: GraphStructure<T> = serde_yaml::from_str(path).unwrap();
    // println!("DESER\n{:?}", deserialized);
    deserialized
}


// --> BREADTH FIRST SEARCH
// Use a list that stores nodes that need to be browsed.
// In one iteration of the algorythm:
// - if the list is not empty, the node is extracted from the list
// - the extracted node is visited (processed)
// - all of the children are placed into the list

pub fn bfs<T>(graph: &Graph<T>, target: Node<T>) -> Option<Vec<Node<T>>> 
where T: PartialEq + Copy + Hash + Eq + Debug {
    let mut visited: HashSet<Node<T>> = HashSet::new();
    let mut history: Vec<Node<T>> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(target);
    queue.push_back(target);
    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        if currentnode == target {
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