#![allow(unused)]
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque, self}, 
    hash::Hash, fmt::{Display, Debug, format}, fs, io::BufRead
};
use serde::{Deserialize, Serialize, de::value};
use std::fmt;

// use anyhow::Result;
use serde_yaml::Value;

#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<Edge<T>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node<T> (pub T);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
    pub fn value(&self) -> T 
    where T: Copy {
        self.0
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStructure {
    pub first_node: String,
    pub second_node: String,
    pub edge: String,
}

fn into_structure<T>(graph: &Graph<T>, i: usize) -> GraphStructure
where T: std::fmt::Display + std::fmt::Debug {
    let triivial_graph = GraphStructure {
        first_node: format!("{:?}", graph.edges[i].0),
        second_node: format!("{:?}", graph.edges[i].1),
        edge: format!("{:?}", graph.edges[i]),
    };
    triivial_graph
}

pub fn serial_triv<T>(graph: &Graph<T>) 
where T: Copy + Display + ToString + std::fmt::Debug {
    let mut result: BTreeMap<String, String> = BTreeMap::new();

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("serial_graph.yml")
        .expect("Couldn't open file");

    let gr_lenght = graph.edges.len();
    for i in 0..gr_lenght {
        let key = format!("Edge {}", i);
        let serialized = serde_yaml::to_string(&into_structure(graph, i))
            .unwrap().clone().into_bytes();
        let serialized: Vec<u8> = serialized
            .into_iter()
            .take_while(|&x| x != 0)
            .collect::<Vec<u8>>();
        
        let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");

        result.insert(key, value_serialized);
    }

    serde_yaml::to_writer(file, &result).unwrap();
}

fn from_structure<T>(graph: String, i: usize) {}

pub fn deserial_triv<T>() 
// -> Graph<T> 
where T: Copy + Display + ToString + std::fmt::Debug {
    // let file = std::fs::OpenOptions::new()
    //     .read(true)
    //     .open("serial_graph.yml")
    //     .expect("Couldn't open file");

    let file = fs::read_to_string("serial_graph.yml").expect("Unable to read line");
    println!("FILE \n{}", file);
    // let file_slice = &file[..];

    let mut all_lines: Vec<String> = vec![];
    let mut index = 0;

    for line in std::io::BufReader::new(std::fs::File::open("serial_graph.yml").expect("Failed at opening file.")).lines() {
        let words = line.unwrap();
        
        // let splitter = format!("Edge {}: |", index);
        // let words_split = words.split(&splitter);

        let edge_index = format!("Edge {}:", index);
        if words.contains(&edge_index) {
            println!("words: \t{}", words);
        }
        all_lines.push(words)
    }
    println!("VACTOR \n{:?}", all_lines);

    let mut some_nodes: Vec<Node<i32>> = vec![];
    let mut some_edges: Vec<Edge<i32>> = vec![];
    let mut edge_index = 0;
    let mut index = 0;
    for mut i in 0..all_lines.len() {
        let edge_index_string = format!("Edge {}:", edge_index);
        if all_lines[i].contains(&edge_index_string) {
            println!("Iteration {}: \t{}", i, &all_lines[index]);
            println!("Iteration {}: \t{}", i, &all_lines[index + 1]);
            println!("Iteration {}: \t{}", i, &all_lines[index + 2]);
            println!("Iteration {}: \t{}", i, &all_lines[index + 3]);
            println!("edge_index {}", edge_index);
            // some_nodes.push(value)
            edge_index += 1;
        } 
        // i += 4
    }

    // let file_str = &file[..];
    // let graph_from_yaml: Graph<T> = serde_yaml::from_str(file_str).unwrap();
    // let graph_from_yaml: Vec<Graph<T>> = serde_yaml::from_str(file_str);

    // return graph
}


// --> BREADTH FIRST SEARCH
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