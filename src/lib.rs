#![allow(unused)]
use std::{
    collections::{BTreeMap, HashSet, VecDeque}, 
    hash::Hash, fmt::{Display, Debug}, io::BufRead
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<Edge<T>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node<T> (pub T);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Edge<T>(pub Node<T>, pub Node<T>);

// I made this struct using generic type (test branch)
// but I can't implement deserialization yet
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStructure {
    pub first_node: String,
    pub second_node: String,
    pub edge: String,
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

impl<T> From<T> for Node<T> {
    fn from(item: T) -> Self {
        Node(item)
    }
}

// 1. CREATE GRAPH

impl<T> Graph<T> {
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge<T>>) -> Self {
        Graph { nodes, edges }
    }
}

// 2. ADD AND REMOVE NODES

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

// 3. ADD AND REMOVE DIRECTED EDGES

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


// 4. SERDE TRIVIAL GRAPH FORMAT
/*
1 First node
2 Second node
#
1 2 Edge between the two
*/

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

pub fn deserial_triv<T>(path: &str) -> Vec<GraphStructure>
where T: Copy + Display + ToString + std::fmt::Debug {
    let mut all_lines: Vec<String> = vec![];

    for line in std::io::BufReader::new(
        std::fs::File::open(path).expect("Failed at opening file.")
    ).lines() {
        let words = line.unwrap();
        all_lines.push(words)
    }
    
    let mut edge_index = 0;

    let mut vec_of_graphs: Vec<GraphStructure> = vec![];
    
    for i in 0..all_lines.len() {
        let edge_index_string = format!("Edge {}: |", edge_index);
        if all_lines[i].contains(&edge_index_string[1..]) {
            let each_part = format!(
                "{}\n{}\n{}", &all_lines[i + 1], &all_lines[i + 2], &all_lines[i + 3]
            );
            let deser: GraphStructure = serde_yaml::from_str(&each_part).unwrap();
            vec_of_graphs.push(deser);
            edge_index += 1;
        }
    }
    
    vec_of_graphs
}


// 5. BREADTH FIRST SEARCH
/*
Use a list that stores nodes that need to be browsed.
In one iteration of the algorythm:
- if the list is not empty, the node is extracted from the list
- the extracted node is visited (processed)
- all of the children are placed into the list
*/

pub fn bfs<T>(graph: &Graph<T>, target: Node<T>) -> Option<Vec<Node<T>>> 
where T: PartialEq + Copy + Hash + Eq + Debug {
    let mut visited: HashSet<Node<T>> = HashSet::new();
    let mut history: Vec<Node<T>> = Vec::new();
    let mut queue: VecDeque<Node<T>> = VecDeque::new();

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