use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::Debug,
};

use crate::node::Node;

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Edge<N>(pub HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>);

impl<N> Edge<N> where N: Clone + Copy + Eq {
    pub fn new(nodes: Node<N>, node_a: N, node_b: N) -> Self {
        let hashed = nodes.0;
        
        // let a: HashMap<&u64, &N> = hashed.iter().map(|(key, value)| {
        //     if value == &node_a { (key, value) }
        //     else { (key, value) }
        // }).collect();
        // let b: HashMap<&u64, &N> = hashed.iter().map(|(key, value)| {
        //     if value == &node_b { (key, value) }
        //     else { (key, value) }
        // }).collect();
        let a_id = hashed.iter()
        .find_map(|(key, &val)| if val == node_a { Some(key) } else { None }).unwrap();
        let b_id = hashed.iter()
        .find_map(|(key, &val)| if val == node_b { Some(key) } else { None }).unwrap();

        let edge_id = format!("1{}{}", a_id, b_id).parse::<u64>().unwrap();

        let mut new_node_a: HashMap<u64, N> = HashMap::new();
        new_node_a.insert(*a_id, node_a);
        let mut new_node_b: HashMap<u64, N> = HashMap::new();
        new_node_b.insert(*b_id, node_b);
        let hash_nodes: (HashMap<u64, N>, HashMap<u64, N>) = (new_node_a, new_node_b);
        let mut hash_edge: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)> = HashMap::new();
        hash_edge.insert(edge_id, hash_nodes);
        let new_edge: Edge<N> = Edge(hash_edge);
        new_edge
    }
}

impl<N> std::fmt::Display for Edge<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{:?}\n-----", self)
    }
}