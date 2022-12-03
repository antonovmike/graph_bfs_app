use graph_library::{Graph, Node, Edge};
use graph_library::{add_node, rem_node};

fn main() {
    let node_a = Node::new("A");
    let node_b = Node::new("B");
    let node_c = Node::new("C");

    let edge_a_b = Edge::new(node_a.clone(), node_b.clone());
    let edge_b_a = Edge::new(node_b.clone(), node_a.clone());
    let edge_a_c = Edge::new(node_a.clone(), node_c.clone());
    let edge_c_a = Edge::new(node_c.clone(), node_a.clone());
    let edge_b_c = Edge::new(node_b.clone(), node_c.clone());
    let edge_c_b = Edge::new(node_c.clone(), node_b.clone());
    
    let vec_of_nodes = vec![node_a, node_b, node_c];
    let vec_of_edges = vec![edge_a_b, edge_b_a, edge_a_c, edge_c_a, edge_b_c, edge_c_b];
    let gr_0 = Graph::new(vec_of_nodes, vec_of_edges);
    println!("Graph 0 NODES:\n{:?}", gr_0.nodes);
    // println!("Graph 0 EDGES:\n{:?}", gr_0.edges);

    let node_d = Node::new("D");
    println!("node_d: {:?}", node_d);
    add_node(gr_0.clone(), node_d);
    println!("Graph 0 + new node:\n{:?}", gr_0.nodes);
}
