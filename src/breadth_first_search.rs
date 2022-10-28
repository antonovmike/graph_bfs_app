// BREADTH FIRST SEARCH

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct Node(u32);
pub struct Edge(u32, u32);

impl Graph {
    pub fn new(node: Vec<Node>, edge: Vec<Edge>) -> Self {
        Graph { nodes: node, edges: edge }
    }
}

pub fn breadth_first_search(gr: Graph) {
    // todo
}