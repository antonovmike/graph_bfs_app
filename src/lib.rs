use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display, format},
    sync::atomic::{AtomicUsize, Ordering}, fs::File, io::{BufReader, BufRead},
};
use node::Node;
// use edge::Edge;

pub mod node;
pub mod edge;
pub mod search;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Graph<N> {
    pub nodes: HashMap<u64, N>,
    pub edges: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>,
    pub root: Option<u64>
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn set_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}


pub trait AnyType {
    fn type_name(&self) -> &'static str;
}

impl<N> AnyType for N {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<N>()
    }
}


impl<N> Graph<N> where N: Debug + Copy + std::cmp::PartialEq {
    pub fn new(nodes: HashMap<u64, N>, edges: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>) -> Self {
        Graph { nodes, edges, root: None }
    }

    // Check if the node exists
    pub fn check_node(&self, add_node: Node<N>) -> bool
    where N: Copy + Eq {
        let mut b = 0;
        if add_node.0.is_empty() {
            b = 0
        } else {
            for (_k, v) in add_node.0 {
                let map = self.nodes.clone();
                let a = map.iter().find_map(|(_key, &val)| if val == v { Some(v) } else { None }).unwrap();
                let x = Some(a);

                if let Some(_value) = x { b = 1 }
                else { b = 0 }
            }            
        }
        if b == 1 { true } else { false }
    }

    pub fn add_node(&mut self, add_node: Node<N>) -> &Graph<N> 
    where N: Copy + Eq
    {
        for (k, v) in add_node.0 {
            if if_gr_contains(self, v) {
                // implement a warning
            } else {
                self.nodes.insert(k, v);
            }
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

    pub fn remove_node(&mut self, node_to_remove: N) -> String where N: Copy + PartialEq + Display {
        let nodes_id = self.get_id(node_to_remove);
        let unwrapped: u64;
        if nodes_id.is_some() { unwrapped = nodes_id.unwrap() }
        else { return format!("Node {} does not exist", node_to_remove) }

        self.nodes.remove(&unwrapped);
        format!("Node {} has been removed", node_to_remove)

        // Check and remove edges containing removed Node
    }

    pub fn set_root(&mut self, root: Option<u64>) -> Option<u64> {
        let ids = &self.nodes;
        let a = root.unwrap();
        if ids.contains_key(&a) {
            self.root = root;
            root
        } else {
            self.root = None;
            root
        }
    }

    pub fn in_graph(&self, index: &u64) -> bool {
        let mut result = false;
        for node in self.nodes.iter() {
            if node.0 == index {
                result = true;
            }
        }
        result
    }

    pub fn get_edge() {}

    pub fn get_node(&self, index: &u64) -> Option<Node<N>> where N: Debug + Copy {
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        for node in self.nodes.iter() {
            if node.0 == index {
                hash_node.insert(*node.0, *node.1);
            }
        }
        Some(Node(hash_node))
    }

    pub fn get_id(&self, node: N) -> Option<u64> where N: Copy + PartialEq {
        let mut result_id = Some(0u64);
        for id in self.nodes.iter() {
            let existing_id = id.0;
            let existing_node = id.1;
            
            if *existing_node == node {
                result_id = Some(*existing_id);
                break;
            } else {
                result_id = None
            }
        }
        result_id
    }

/* SERDE TRIVIAL GRAPH FORMAT:
1 First node
2 Second node
#
1 2 Edge between the two */
pub fn serial_triv(graph: &Graph<N>, path: &str) where
N: Serialize + Copy + Display + ToString + std::fmt::Debug
{
    let first_node: N = graph.nodes[&0];
    let mut str_nodes = "Nodes:\n".to_string();
    let mut str_edges = "Edges:\n".to_string();
    let type_of_node: &str = first_node.type_name();

    let path = format!("{}/serial_graph.yml", path);
    let _file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Couldn't open file");
    
    for i in 0..graph.nodes.len() {
        let index = i as u64;

        // if Graph::get_node(graph, &index).is_some() {

        // } else {
        //     continue;
        // }
        
        let temp_node = Graph::get_node(graph, &index).unwrap();
        let t_node = temp_node.clone().0[&index];
        let temp_id = Graph::get_id(graph, t_node).unwrap();
        // println!("temp_id: {}; temp_node: {}", temp_id, t_node);
        let temp_node = format!("{}: {:?}\n", temp_id, temp_node.0[&index]);
        str_nodes.push_str(&temp_node)
    }

    let mut str = "".to_string();

    if graph.edges.is_empty() {
        str = format!("{}\n{}", type_of_node, str_nodes);
    } else {
        str = format!("{}\n{}\n#\n{}", type_of_node, str_nodes, str_edges);
    }

    serde_yaml::to_writer(file, &str).unwrap();
}
/* 
    pub fn serial_triv(graph: &Graph<N>, path: &str) where
    N: Serialize + Copy + Display + ToString + std::fmt::Debug
    {
        let first_node: N = graph.nodes[&0];
        let type_of_node: &str = first_node.type_name();
        println!("serial_triv Node: {}", type_of_node);

        let path = format!("{}/serial_graph.yml", path);
        let _file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("Couldn't open file");
        
        let serialized = serde_yaml::to_string(graph)
            .unwrap()
            .into_bytes();

        let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");
        let additional_info = format!("Type: {}\n{}", type_of_node, value_serialized);

        serde_yaml::to_writer(file, &additional_info).unwrap();
    }
*/

    pub fn deserial_triv(&self, graph: &mut Graph<N>, path: &str) -> Result<(), String> {
        let input = File::open(path).expect("Could Not Open a File to Read From");
        let buf = BufReader::new(input);
        let mut edges = false;
        // Iterate over lines
        for line in buf.lines().map(|line| line.unwrap()) {
            let parts: Vec<&str> = line.split(" ").collect();
            
            if !edges {
                if parts.get(0).unwrap() == &"#" {
                    edges = true;
                    continue;
                }
                let label = parts[1..].join(" ");
                let splitted = label.split(" ").collect::<Vec<&str>>();
                if splitted.len() == 5 {
                    println!("deserial NODE: {}", splitted[4])
                }
                if splitted.len() == 6 {
                    println!("deserial EDGE: {}", splitted[5])
                }
                // println!("LABEL: {}", label);
            } else {
                // Edge::new()?;
            }
        }

        Ok(())
    }
}

pub fn if_gr_contains<N>(graph: &Graph<N>, node: N) -> bool where N: Copy + Eq {
    let mut indicator = true;
    for i in graph.nodes.iter() {
        let node_names = graph.nodes[i.0];
        if node_names == node {
            indicator= true;
            break;
        } else { indicator = false }
    }
    indicator
}

impl<N> std::fmt::Display for Graph<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nGRAPH:\n nodes: {:?}\n edges: {:?}\n root: {:?}\n-----",
            self.nodes, self.edges, self.root
        )
    }
}


pub fn type_finder() -> String {
    let path = "serde/serial_graph.yml";
    let input = File::open(path).expect("Could Not Open a File to Read From");
    let buf = BufReader::new(input);

    let mut result = "".to_string();
    
    for (index, line) in buf.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if index == 1 {
            result = line.trim().to_string();
        }
    }
    result
}