use graph_library::*;

fn main() {
    // Create nodes - impl Node
    let list_of_nodes = ["A", "B", "C", "D"];
    let nodes = Node::new(&list_of_nodes);
    println!("{}", nodes);

    // Create nodes - impl Graph
    let second_list = ["F", "G"];
    let nodes_in_gr = Graph::create_node(&second_list);
    println!("{}", nodes_in_gr);

    // Create edges
    let edge_a_b = Edge::new(nodes.clone(), "A", "B");
    println!("{}", edge_a_b);
    let edge_b_a = Edge::new(nodes.clone(), "B", "A");
    println!("{}", edge_b_a);

    let mut gr_0 = Graph::new(nodes.0, edge_a_b.0);
    println!("{}", gr_0);

    let node_e = Node::new(&["C"]);
    let gr_0 = Graph::add_node(&mut gr_0, node_e.clone());
    println!("{}", gr_0);

    let node_to_get: Node<&str> = Graph::get_node(&gr_0.clone(), &1);
    println!("get node: {}", node_to_get);

    // let node_f = Node::new(&["F"]);
    println!("Check if node exist: {}", Graph::check_node(&gr_0, node_e));
}