#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet, VecDeque, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
    io::BufRead, sync::atomic::{AtomicUsize, Ordering},
};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Graph<N> {
    pub nodes: HashMap<u64, N>,
    pub edges: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>,
}

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Node<N>(pub HashMap<u64, N>);

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Edge<N>(pub HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>);


static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn set_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}


// 1. CREATE GRAPH
impl<N> Node<N> where N: Copy {
    pub fn new(list_of_nodes: &[N]) -> Self {
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        let mut index = 0;
        for _i in list_of_nodes.iter() {
            let id = set_id() as u64;
            hash_node.insert(id, list_of_nodes[index]);
            index += 1;
        }
        let new_node: Node<N> = Node(hash_node);
        new_node
    }
}

impl<N> Edge<N> where N: Clone + Copy + Eq {
    pub fn new(nodes: Node<N>, node_a: N, node_b: N) -> Self {
        let mut index: u64 = 0;
        let hashed = nodes.0;
        
        let a: HashMap<&u64, &N> = hashed.iter().map(|(key, value)| {
            if value == &node_a { (key, value) }
            else { (key, value) }
        }).collect();
        let b: HashMap<&u64, &N> = hashed.iter().map(|(key, value)| {
            if value == &node_b { (key, value) }
            else { (key, value) }
        }).collect();
        let a_id = hashed.iter()
        .find_map(|(key, &val)| if val == node_a { Some(key) } else { None }).unwrap();
        let b_id = hashed.iter()
        .find_map(|(key, &val)| if val == node_b { Some(key) } else { None }).unwrap();

        let edge_id = format!("1{}{}", a_id, b_id).parse::<u64>().unwrap();

        let mut new_node_a: HashMap<u64, N> = HashMap::new();
        new_node_a.insert(*a_id, node_a);
        let mut new_node_b: HashMap<u64, N> = HashMap::new();
        new_node_b.insert(*b_id, node_b);
        let mut hash_nodes = (new_node_a, new_node_b);
        let mut hash_edge: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)> = HashMap::new();
        hash_edge.insert(edge_id, hash_nodes);
        let new_edge: Edge<N> = Edge(hash_edge);
        new_edge
    }
}

impl<N> Graph<N> 
where N: Debug + Copy
{
    pub fn new(nodes: HashMap<u64, N>, edges: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>) -> Self {
        Graph { nodes, edges }
    }
    // Check if node exists in the graph
    pub fn in_graph(&self, index: usize) -> bool {
        false
    }
    // Check if the node exists
    pub fn check_node(&self, add_node: Node<N>) -> bool
    where N: Copy + Eq {
        let mut b = 0;
        if add_node.0.len() == 0 {
            b = 0
        } else {
            for (k, v) in add_node.0 {
                let map = self.nodes.clone();
                let a = map.iter().find_map(|(key, &val)| if val == v { Some(v) } else { None }).unwrap();
                let x = Some(a);

                if let Some(value) = x { b = 1 }
                else { b = 0 }
            }
        }
        if b == 1 { true } else { false }
    }

    pub fn add_node(&mut self, add_node: Node<N>) -> &Graph<N> 
    where N: Copy + Eq
    {
        for (k, v) in add_node.0 {
            // if self.get_node(&k).0.get_key_value(&k).unwrap().0 == &k {
            //     self.nodes.insert(k, v);
            // }

            // let map = self.nodes.clone();
            // let a = map.iter().find_map(|(key, &val)| if val == v { Some(v) } else { None }).unwrap();

            self.nodes.insert(k, v);
        };
        self
    }

    pub fn create_node(list_of_nodes: &[N]) -> Node<N> {
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        let mut index = 0;
        for _i in list_of_nodes.iter() {
            let id = set_id() as u64;
            hash_node.insert(id, list_of_nodes[index]);
            index += 1;
        }
        let new_node: Node<N> = Node(hash_node);
        new_node
    }

    pub fn get_node(&self, index: &u64) -> Node<N> {
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        for node in self.nodes.iter() {
            if node.0 == index {
                hash_node.insert(*node.0, *node.1);
            }
        }
        Node(hash_node)
    }

/* SERDE TRIVIAL GRAPH FORMAT:
1 First node
2 Second node
#
1 2 Edge between the two */

    pub fn serial_triv(graph: &Graph<N>) where
    N: Serialize + Copy + Display + ToString + std::fmt::Debug
    {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("serial_graph.yml")
            .expect("Couldn't open file");
        
        let serialized = serde_yaml::to_string(graph)
            .unwrap()
            .into_bytes();

        let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");

        serde_yaml::to_writer(file, &value_serialized).unwrap();
    }
}


impl<N> std::fmt::Display for Graph<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nGRAPH:\n nodes: {:?}\n edges: {:?}\n-----",
            self.nodes, self.edges,
        )
    }
}

impl<N> std::fmt::Display for Node<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{:?}\n-----", self)
    }
}

impl<N> std::fmt::Display for Edge<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{:?}\n-----", self)
    }
}


pub fn deserial_triv() 
// where N: Deserialize
// where N: DeserializeOwned
{
    let string = std::fs::read_to_string("serial_graph.yml").expect("Error in reading the file");
    
    // let deserialized: Graph<&'a str> = serde_yaml::from_str(&str[..]).unwrap();
}