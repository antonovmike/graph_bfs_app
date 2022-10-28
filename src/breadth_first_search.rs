// BREADTH FIRST SEARCH

pub struct Graph {
    pub nodes: Vec<u32>,
    pub edges: Vec<(u32, u32)>,
}

impl Graph {
    pub fn new(node: Vec<u32>, edge: Vec<(u32, u32)>) -> Self {
        Graph { nodes: node, edges: edge }
    }
}

pub fn breadth_first_search(gr: Graph) {
    // todo
}