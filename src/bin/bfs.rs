use graph_library::{Graph, Node, Edge, add_edge, rem_edge};
use graph_library::{add_node, rem_node};

fn main() {
    // Create nodes
    let node_a = Node::new("A");
    let node_b = Node::new("B");
    let node_c = Node::new("C");

    // Create edges
    let edge_a_b = Edge::new(node_a.clone(), node_b.clone());
    let edge_b_a = Edge::new(node_b.clone(), node_a.clone());
    let edge_a_c = Edge::new(node_a.clone(), node_c.clone());
    let edge_c_a = Edge::new(node_c.clone(), node_a.clone());
    let edge_b_c = Edge::new(node_b.clone(), node_c.clone());
    let edge_c_b = Edge::new(node_c.clone(), node_b.clone());
    
    // 1. Create graph
    let vec_of_nodes = vec![node_a, node_b, node_c];
    let vec_of_edges = vec![edge_a_b, edge_b_a, edge_a_c, edge_c_a, edge_b_c];
    let gr_0 = Graph::new(vec_of_nodes, vec_of_edges);
    // println!("Graph 0 NODES:\n{:?}", gr_0.nodes);
    // println!("Graph 0 EDGES:\n{:?}", gr_0.edges);

    // 2. ADD AND REMOVE NODES
    // works, but creates a new graph
    let node_d = Node::new("D");
    let gr_1 = add_node(gr_0.clone(), node_d.clone());
    // println!("Graph 1 + node_d:\n{:?}", gr_1.nodes);
    let gr_2 = rem_node(gr_0.clone(), node_d);
    // println!("Graph 2 - node_d:\n{:?}", gr_2.nodes);

    // 3. ADD AND REMOVE DIRECTED EDGES
    // works, but creates a new graph
    let gr_3 = add_edge(gr_0.clone(), edge_c_b.clone());
    println!("Graph 3 + edge_c_b:\n{:?}", gr_3.edges[5]);
    let gr_4 = rem_edge(gr_0, edge_c_b);
    println!("Graph 4 - edge_c_b:\n{:?}", gr_4.edges);

    // 4. SERDE TRIVIAL GRAPH FORMAT

    // 5. BREADTH FIRST SEARCH
    
}
