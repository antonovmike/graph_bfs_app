use graph_bfs_app::Graph;
use graph_bfs_app::rem_node;
use graph_bfs_app::add_node;
use graph_bfs_app::rem_edge;
use graph_bfs_app::add_edge;
use graph_bfs_app::bfs;
use graph_bfs_app::serial_triv;
use graph_bfs_app::{Node, Edge};

fn main() {
    // --> CREATE NEW GRAPH
    let some_nodes: Vec<Node<i32>> = vec![Node(1), Node(2), Node(3), Node(4)];
    let some_edges: Vec<Edge> = vec![Edge(1, 2), Edge(3, 4)];
    let gr_0 = Graph::new(some_nodes, some_edges);
    println!("Graph new nodes: \t{:?}", gr_0.nodes);
    println!("Graph new edges: \t{:?}", gr_0.edges);

    // --> ADD AND REMOVE NODES
    let seven = Node(7);
    let gr_1 = add_node(gr_0, seven);
    println!("One node added: \t{:?}", gr_1.nodes);
    let gr_2 = rem_node(gr_1, Node(7));
    println!("One node removed: \t{:?}", gr_2.nodes);

    // --> ADD AND REMOVE DIRECTED EDGES
    let two_three = Edge(2, 3);
    // let three_two = Edge(3, 2);
    let gr_3 = add_edge(gr_2, two_three.clone());
    println!("One edge added: \t{:?}", gr_3.edges);
    let gr_4 = rem_edge(gr_3, two_three);
    println!("One edge removed: \t{:?}", gr_4.edges);

    let gr_5 = add_edge(gr_4, two_three.clone());

    // --> SERDE INTO TRIVIAL GRAPH FORMAT
    // ...
    serial_triv(&gr_5);

    // --> BREADTH FIRST SEARCH
    let found = bfs(&gr_5, Node(0), Node(2));
    println!("BFS: \t\t\t{:?}", found);
}
