use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::Debug,
};

use crate::set_id;

#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Node<N>(pub HashMap<u64, N>);

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

impl<N> std::fmt::Display for Node<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{:?}\n-----", self)
    }
}