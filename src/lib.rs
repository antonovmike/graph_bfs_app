#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet, VecDeque, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
    io::BufRead, sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Serialize, Clone)]
pub struct Graph<N> {
    pub nodes: Vec<Node<N>>,
    pub edges: Vec<Edge<N>>,
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Node<N>(pub HashMap<u64, N>);
#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Edge<N>(pub HashMap<u64, (Node<N>, Node<N>)>);

// impl<'a, N: 'a> Copy for Node<'a, N> where N: Copy {}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn set_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

// 1. CREATE GRAPH

impl<N> Node<N> {
    pub fn new(node: N) -> Self {
        let id = set_id() as u64;
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        hash_node.insert(id, node);
        let new_node: Node<N> = Node(hash_node);
        new_node
    }

    // pub fn value(&self) -> Node<N>
    // where
    //     N: Copy,
    // {
    //     Node(self.0)
    // }

    // pub fn neighbors(&self, graph: &Graph<N>) -> Vec<Node<N>>
    // where
    //     N: PartialEq + Copy + Hash,
    // {
    //     graph
    //         .nodes
    //         .iter()
    //         .filter(|e| e.0 == self.0)
    //         .map(|e| e.0.into())
    //         .collect()
    // }
}

impl<N> Edge<N> where N: Clone {
    pub fn new(node_a: Node<N>, node_b: Node<N>) -> Self {
        let a_clone = node_a.clone();
        let b_clone = node_b.clone();
        let hash_a = a_clone.0;
        let hash_b = b_clone.0;
        
        let mut id_vec: Vec<u64> = vec![];
        for (key, val) in hash_a.iter() {
            id_vec.push(*key);
        }
        for (key, val) in hash_b.iter() {
            id_vec.push(*key);
        }
        let edge_id = format!("1{}{}", id_vec[0], id_vec[1]).parse::<u64>().unwrap();

        let mut hash_nodes = (node_a, node_b);
        let mut hash_edge: HashMap<u64, (Node<N>, Node<N>)> = HashMap::new();
        hash_edge.insert(edge_id, hash_nodes);
        let new_edge: Edge<N> = Edge(hash_edge);
        new_edge
    }
}

impl<N> Graph<N> 
where N: Debug + Copy
{
    pub fn new(nodes: Vec<Node<N>>, edges: Vec<Edge<N>>) -> Self {
        Graph { nodes, edges }
    }
    // Check if node exists in the graph
    pub fn in_graph(&self, index: usize) -> bool {
        false
    }
    // Add new node
    pub fn add_node(&mut self, add_node: Node<N>) -> &Graph<N> 
    where N: Copy 
    {
        let mut new_vec = self;
        new_vec.nodes.push(add_node);
        new_vec
    }
}

// 2. ADD AND REMOVE NODES

pub fn add_node<N>(graph: Graph<N>, add_node: Node<N>) -> Graph<N> 
where N: Copy 
{
    let mut new_vec = graph;
    new_vec.nodes.push(add_node);
    new_vec
}

pub fn rem_node<N>(graph: Graph<N>, remove_node: Node<N>) -> Graph<N>
where
    N: PartialEq,
{
    let mut nodes = graph.nodes;
    nodes.retain(|value: &Node<N>| *value != remove_node);
    let new_vec: Graph<N> = Graph {
        nodes,
        edges: graph.edges,
    };
    new_vec
}

// 3. ADD AND REMOVE DIRECTED EDGES

pub fn add_edge<N>(graph: Graph<N>, add_edge: Edge<N>) -> Graph<N> {
    let mut new_vec = graph;
    new_vec.edges.push(add_edge);
    new_vec
}

pub fn rem_edge<N>(graph: Graph<N>, remove_edge: Edge<N>) -> Graph<N>
where
    N: PartialEq,
{
    let mut edges: Vec<Edge<N>> = graph.edges;
    edges.retain(|value: &Edge<N>| *value != remove_edge);
    Graph {
        nodes: graph.nodes,
        edges,
    }
}

// 4. SERDE TRIVIAL GRAPH FORMAT
/*
1 First node
2 Second node
#
1 2 Edge between the two
*/

pub fn serial_triv<N>(graph: &Graph<N>) where
N: Serialize + Copy + Display + ToString + std::fmt::Debug
{
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("serial_graph.yml")
        .expect("Couldn't open file");
    
    let serialized = serde_yaml::to_string(graph)
        .unwrap()
        .clone()
        .into_bytes();

    let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");

    serde_yaml::to_writer(file, &value_serialized).unwrap();
}

pub fn deserial_triv<N>() 
// where N: Deserialize
{
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("serial_graph.yml")
        .expect("Couldn't open file");
    // let work_from_json: Graph<N> = serde_yaml::from_str(file.as_str()).unwrap();
}

// 5. BREADTH FIRST SEARCH
/*
Use a list that stores nodes that need to be browsed.
In one iteration of the algorythm:
- if the list is not empty, the node is extracted from the list
- the extracted node is visited (processed)
- all of the children are placed into the list
*/

pub fn bfs<N>(graph: &Graph<N>, target: Node<N>) 
// -> HashMap<u64, N> 
-> Option<Vec<Node<N>>>
where
    N: PartialEq + Copy + Hash + Eq + Debug,
{
    /*
    // Target node. For example: {4: "D"}
    let mut target_node: HashMap<u64, N> = HashMap::new();
    let tn = target.0;
    for (key, val) in tn.iter() {
        target_node.insert(*key, *val);
    }
    // List of nodes. For example: {2: "C", 0: "A", 1: "B"}
    let mut all_nodes: HashMap<u64, N> = HashMap::new();
    let an = &graph.nodes;
    for i in an {
        let c = i.clone().0;
        for (key, val) in c.iter() {
            all_nodes.insert(*key, *val);
        }
    }

    let mut history: Vec<Node<N>> = Vec::new();
    let mut queue: VecDeque<Node<N>> = VecDeque::new();
    */

    // let mut visited: HashMap<u64, N> = HashMap::new();
    // let mut history: Vec<Node<N>> = Vec::new();
    // let mut queue: VecDeque<Node<N>> = VecDeque::new();

    // let key_val = target.0.get_key_value(&0).unwrap();
    // visited.insert(*key_val.0, *key_val.1);
    // queue.push_back(target);
    // while let Some(currentnode) = queue.pop_front() {
    //     history.push(currentnode.value());

    //     if currentnode == target.0 {
    //         return Some(history);
    //     }

    //     for neighbor in currentnode.neighbors(graph) {
    //         if !visited.contains(&neighbor) {
    //             visited.insert(neighbor);
    //             queue.push_back(neighbor);
    //         }
    //     }
    // }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_str() {
        let node_a = Node::new("A").0;
        let mut example: HashMap<u64, &str> = HashMap::new();
        example.insert(1, "A");
        assert_eq!(example, node_a);
    }
    #[test]
    fn create_node_char() {
        let node_a = Node::new('A').0;
        let mut example: HashMap<u64, char> = HashMap::new();
        example.insert(0, 'A');
        assert_eq!(example, node_a);
    }
    #[test]
    fn create_node_string() {
        let node_a = Node::new("A".to_string()).0;
        let mut example: HashMap<u64, String> = HashMap::new();
        example.insert(2, "A".to_string());
        assert_eq!(example, node_a);
    }
}