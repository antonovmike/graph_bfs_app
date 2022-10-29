use graph_bfs_app::Graph;
use graph_bfs_app::rem_node;
use graph_bfs_app::add_node;
use graph_bfs_app::rem_edge;
use graph_bfs_app::add_edge;

fn main() {
    let gr_0: Graph<u8, (u8, u8)> = Graph {
        nodes: vec![1, 2, 3 , 4],
        edges: vec![(1, 2), (3, 4)],
    };

    // --> ADD AND REMOVE NODES
    let gr_1: Graph<u8, (u8, u8)> = add_node(gr_0, 7);
    println!("One node added: {:?}", gr_1.nodes);
    let gr_2: Graph<u8, (u8, u8)> = rem_node(gr_1, 7);
    println!("One node removed: {:?}", gr_2.nodes);

    // --> ADD AND REMOVE DIRECTED EDGES
    let gr_3: Graph<u8, (u8, u8)> = add_edge(gr_2, (2, 3));
    println!("One edge added: {:?}", gr_3.edges);
    let gr_4: Graph<u8, (u8, u8)> = rem_edge(gr_3, (2, 3));
    println!("One edge added: {:?}", gr_4.edges);
}
