use graph_library::{Graph, Node, Edge};
use graph_library::{
    add_node, rem_node,  
    add_edge, rem_edge,  
    serial_triv, deserial_triv,
    bfs
};

fn main() {
    // 1. CREATE NEW GRAPH
    println!("\t1. CREATE NEW GRAPH");
    let some_nodes: Vec<Node<i32>> = vec![Node(1), Node(2), Node(3), Node(4)];
    let some_edges: Vec<Edge<i32>> = vec![Edge(Node(1), Node(2)), Edge(Node(3), Node(4))];
    let gr_0 = Graph::new(some_nodes, some_edges);
    println!("Graph new nodes: \t{:?}", gr_0.nodes);
    println!("Graph new edges: \t{:?}\n", gr_0.edges);

    // 2. ADD AND REMOVE NODES
    println!("\t2. ADD AND REMOVE NODES");
    let seven = Node(7);
    let gr_1 = add_node(gr_0, seven);
    println!("One node added: \t{:?}", gr_1.nodes);
    let gr_2 = rem_node(gr_1, Node(7));
    println!("One node removed: \t{:?}\n", gr_2.nodes);

    // 3. ADD AND REMOVE DIRECTED EDGES
    println!("\t3. ADD AND REMOVE DIRECTED EDGES");
    let two_three = Edge(Node(2), Node(3));
    let gr_3 = add_edge(gr_2, two_three.clone());
    println!("One edge added: \t{:?}", gr_3.edges);
    let gr_4 = rem_edge(gr_3, two_three);
    println!("One edge removed: \t{:?}\n", gr_4.edges);

    // 4. SERDE TRIVIAL GRAPH FORMAT
    println!("\t4. SERDE TRIVIAL GRAPH FORMAT");
    let gr_5 = add_edge(gr_4, two_three.clone());
    serial_triv(&gr_5);
    let deserialized_gr = deserial_triv::<i32>("serial_graph.yml");
    println!("Deserialized\t\t{:?}\n", deserialized_gr);

    // 5. BREADTH FIRST SEARCH
    println!("\t5. BREADTH FIRST SEARCH");
    let found = bfs(&gr_5, Node(2));
    println!("BFS: \t\t\t{:?}", found.unwrap());
}
