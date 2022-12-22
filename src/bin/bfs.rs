use graph_library::*;
use graph_library::node::Node;
use graph_library::edge::Edge;

fn main() {
    // Create nodes - impl Node
    let list_of_nodes = ["A", "B", "C", "D"];
    let nodes = Node::new(&list_of_nodes);
    // println!("{}", nodes);

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

    // let node_e = Node::new(&["E"]);
    // let gr_0 = Graph::add_node(&mut gr_0, node_e.clone());
    // println!("{}", gr_0);

    let root = Graph::set_root(&mut gr_0, Some(2));
    println!("Root is set to {:?}", root);
    println!("{}", gr_0);

    let getid = Graph::get_id(&gr_0, "C");
    println!("GET ID = {:?}", getid);

    let removed_node = Graph::remove_node(&mut gr_0, "B");
    println!("REMOVED NODE: {:?}", removed_node);
    println!("REMOVED NODE: {}", gr_0);

    // let node_to_get: Node<&str> = Graph::get_node(&gr_0, &1);
    // println!("get node: {}", node_to_get);

    // let node_f = Node::new(&["F"]);
    // println!("Check if node exist: {}", Graph::check_node(&gr_0, node_f));

    // let node_f = Node::new(&["A"]);
    // let gr_1 = Graph::add_node(&mut gr_0, node_f.clone());
    // println!("{}", gr_1);

    Graph::serial_triv(&gr_0, "serde");


    let mut gr_1 = gr_0.clone();
    Graph::deserial_triv(&gr_0, &mut gr_1, "serde/serial_graph.yml");
}