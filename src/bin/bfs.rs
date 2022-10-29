use graph_bfs_app::Graph;
use graph_bfs_app::test;
use graph_bfs_app::add_node;

fn main() {
    let test = test(2, 2);
    println!("TEST: {}", test);

    let gr_0: Graph<u8, (u8, u8)> = Graph {
        nodes: vec![1, 2],
        edges: vec![(1, 2)],
    };
}
