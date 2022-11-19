#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet, VecDeque, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
    io::BufRead,
};

#[derive(Clone)]
// pub struct Graph<T> {
//     pub nodes: Vec<Node<T>>,
//     pub edges: Vec<Edge<T>>,
// }
struct Graph<N, E> {
    nodes: HashMap<u64, N>,
    edges: HashMap<u64, HashMap<u64, E>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node<N>(pub HashMap<u64, N>);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Edge<N, E>(pub HashMap<u64, HashMap<u64, E>>);

// I made this struct using generic type (test branch)
// but I can't implement deserialization yet
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStructure {
    pub first_node: String,
    pub second_node: String,
    pub edge: String,
}

impl<N, E> Node<N> {
    pub fn value(&self) -> Node<N>
    where
        N: Copy,
    {
        Node(self.0)
    }

    pub fn neighbors(&self, graph: &Graph<N, E>) -> Vec<Node<N>>
    where
        N: PartialEq + Copy + Hash,
    {
        graph
            .nodes
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.0.into())
            .collect()
    }
}

impl<N> From<N> for Node<N> {
    fn from(item: N) -> Self {
        Node(item)
    }
}

// 1. CREATE GRAPH

impl<N, E> Graph<N, E> {
    pub fn new(nodes: Vec<Node<N>>, edges: Vec<Edge<N, E>>) -> Self {
        Graph { nodes, edges }
    }
}

// 2. ADD AND REMOVE NODES

pub fn add_node<N, E>(graph: Graph<N, E>, to_add: Node<N>) -> Graph<N, E> {
    let mut new_vec = graph;
    new_vec.nodes.push(to_add);
    new_vec
}

pub fn rem_node<N, E>(graph: Graph<N, E>, to_remove: Node<N>) -> Graph<N, E>
where
    N: PartialEq,
{
    let mut nodes = graph.nodes;
    nodes.retain(|value: &Node<N>| *value != to_remove);
    let new_vec: Graph<N, E> = Graph {
        nodes,
        edges: graph.edges,
    };
    new_vec
}

// 3. ADD AND REMOVE DIRECTED EDGES

pub fn add_edge<N, E>(graph: Graph<N, E>, to_add: Edge<N, E>) -> Graph<N, E> {
    let mut new_vec = graph;
    new_vec.edges.push(to_add);
    new_vec
}

pub fn rem_edge<N, E>(graph: Graph<N, E>, to_remove: Edge<N, E>) -> Graph<N, E>
where
    E: PartialEq,
{
    let mut edges: Vec<Edge<E>> = graph.edges;
    edges.retain(|value: &Edge<E>| *value != to_remove);
    Graph {
        nodes: graph.nodes,
        edges,
    }
}

// 4. SERDE TRIVIAL GRAPH FORMAT
/*
1 First node
2 Second node
#
1 2 Edge between the two
*/

fn into_structure<N, E>(graph: &Graph<N, E>, i: usize) -> GraphStructure
where
    E: std::fmt::Display + std::fmt::Debug,
{
    GraphStructure {
        first_node: format!("{:?}", graph.edges[i].0),
        second_node: format!("{:?}", graph.edges[i].1),
        edge: format!("{:?}", graph.edges[i]),
    }
}

pub fn serial_triv<N, E>(graph: &Graph<N, E>)
where
    N: Copy + Display + ToString + std::fmt::Debug,
{
    let mut result: BTreeMap<String, String> = BTreeMap::new();

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("serial_graph.yml")
        .expect("Couldn't open file");

    let gr_lenght = graph.edges.len();
    for i in 0..gr_lenght {
        let key = format!("Edge {}", i);
        let serialized = serde_yaml::to_string(&into_structure(graph, i))
            .unwrap()
            .clone()
            .into_bytes();
        let serialized: Vec<u8> = serialized
            .into_iter()
            .take_while(|&x| x != 0)
            .collect::<Vec<u8>>();

        let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");

        result.insert(key, value_serialized);
    }

    serde_yaml::to_writer(file, &result).unwrap();
}

pub fn deserial_triv<T>(path: &str) -> Vec<GraphStructure>
where
    T: Copy + Display + ToString + std::fmt::Debug,
{
    let mut all_lines: Vec<String> = vec![];

    for line in
        std::io::BufReader::new(std::fs::File::open(path).expect("Failed at opening file.")).lines()
    {
        let words = line.unwrap();
        all_lines.push(words)
    }

    let mut edge_index = 0;

    let mut vec_of_graphs: Vec<GraphStructure> = vec![];

    for i in 0..all_lines.len() {
        let edge_index_string = format!("Edge {}: |", edge_index);
        if all_lines[i].contains(&edge_index_string[1..]) {
            let each_part = format!(
                "{}\n{}\n{}",
                &all_lines[i + 1],
                &all_lines[i + 2],
                &all_lines[i + 3]
            );
            let deser: GraphStructure = serde_yaml::from_str(&each_part).unwrap();
            vec_of_graphs.push(deser);
            edge_index += 1;
        }
    }

    vec_of_graphs
}

// 5. BREADTH FIRST SEARCH
/*
Use a list that stores nodes that need to be browsed.
In one iteration of the algorythm:
- if the list is not empty, the node is extracted from the list
- the extracted node is visited (processed)
- all of the children are placed into the list
*/

pub fn bfs<N, E>(graph: &Graph<N, E>, target: Node<N>) -> Option<Vec<Node<N>>>
where
    N: PartialEq + Copy + Hash + Eq + Debug,
{
    let mut visited: HashSet<Node<N>> = HashSet::new();
    let mut history: Vec<Node<N>> = Vec::new();
    let mut queue: VecDeque<Node<N>> = VecDeque::new();

    visited.insert(target);
    queue.push_back(target);
    while let Some(currentnode) = queue.pop_front() {
        history.push(currentnode.value());

        if currentnode == target {
            return Some(history);
        }

        for neighbor in currentnode.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn main() {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // 1. CREATE NEW GRAPH
//     #[test]
//     fn create_graph_of_u8() {
//         let some_nodes: Vec<Node<u8>> = vec![Node(1), Node(2), Node(3), Node(4)];
//         let some_edges: Vec<Edge<u8>> = vec![Edge(Node(1), Node(2)), Edge(Node(3), Node(4))];
//         let graph_of_u8 = Graph::new(some_nodes.clone(), some_edges.clone());
//         assert_eq!(some_nodes, graph_of_u8.nodes);
//         assert_eq!(some_edges, graph_of_u8.edges);
//     }
//     #[test]
//     fn create_graph_of_char() {
//         let some_nodes: Vec<Node<char>> = vec![Node('a'), Node('b'), Node('c'), Node('d')];
//         let some_edges: Vec<Edge<char>> =
//             vec![Edge(Node('a'), Node('b')), Edge(Node('c'), Node('d'))];
//         let graph_of_char = Graph::new(some_nodes.clone(), some_edges.clone());
//         assert_eq!(some_nodes, graph_of_char.nodes);
//         assert_eq!(some_edges, graph_of_char.edges);
//     }
//     #[test]
//     fn create_graph_of_str() {
//         let some_nodes: Vec<Node<&str>> = vec![Node("aa"), Node("bb"), Node("cc"), Node("dd")];
//         let some_edges: Vec<Edge<&str>> =
//             vec![Edge(Node("aa"), Node("bb")), Edge(Node("cc"), Node("dd"))];
//         let graph_of_str = Graph::new(some_nodes.clone(), some_edges.clone());
//         assert_eq!(some_nodes, graph_of_str.nodes);
//         assert_eq!(some_edges, graph_of_str.edges);
//     }

//     // 2. ADD AND REMOVE NODES
//     // 3. ADD AND REMOVE DIRECTED EDGES
//     #[test]
//     fn add_nodes_and_edges() {
//         let gr_0 = Graph::new(
//             vec![Node(1), Node(2), Node(3), Node(4)],
//             vec![Edge(Node(1), Node(2)), Edge(Node(3), Node(4))],
//         );
//         let gr_1 = add_node(gr_0, Node(5));
//         let gr_2 = add_edge(gr_1.clone(), Edge(Node(4), Node(5)));
//         let control_nodes: Vec<Node<i32>> = vec![Node(1), Node(2), Node(3), Node(4), Node(5)];
//         let control_edges: Vec<Edge<i32>> = vec![
//             Edge(Node(1), Node(2)),
//             Edge(Node(3), Node(4)),
//             Edge(Node(4), Node(5)),
//         ];
//         assert_eq!(control_nodes, gr_1.nodes);
//         assert_eq!(control_edges, gr_2.edges);
//     }
//     #[test]
//     fn rem_nodes() {
//         let gr_0 = Graph::new(
//             vec![Node(1), Node(2), Node(3), Node(4), Node(5)],
//             vec![
//                 Edge(Node(1), Node(2)),
//                 Edge(Node(3), Node(4)),
//                 Edge(Node(4), Node(5)),
//             ],
//         );
//         let gr_1 = rem_node(gr_0, Node(5));
//         let gr_2 = rem_edge(gr_1.clone(), Edge(Node(4), Node(5)));
//         let control_nodes: Vec<Node<i32>> = vec![Node(1), Node(2), Node(3), Node(4)];
//         let control_edges: Vec<Edge<i32>> = vec![Edge(Node(1), Node(2)), Edge(Node(3), Node(4))];
//         assert_eq!(control_nodes, gr_1.nodes);
//         assert_eq!(control_edges, gr_2.edges);
//     }

//     // 4. SERDE TRIVIAL GRAPH FORMAT
//     #[test]
//     fn serialize_trivial_graph() {
//         std::fs::remove_file("serial_graph.yml");
//         let gr_0 = Graph::new(
//             vec![Node(1), Node(2), Node(3), Node(4)],
//             vec![Edge(Node(1), Node(2)), Edge(Node(3), Node(4))],
//         );
//         serial_triv(&gr_0);
//         let file_content = std::fs::read_to_string("serial_graph.yml").expect("Couldn't open file");
//         let control_content = "Edge 0: |
//   first_node: Node(1)
//   second_node: Node(2)
//   edge: Edge(Node(1), Node(2))
// Edge 1: |
//   first_node: Node(3)
//   second_node: Node(4)
//   edge: Edge(Node(3), Node(4))
// "
//         .to_string();
//         assert_eq!(file_content, control_content);
//     }

//     #[test]
//     fn deserialize_trivial_graph() {
//         let deserialized_gr = deserial_triv::<i32>("serial_graph.yml");
//         let control_content = vec![
//             GraphStructure {
//                 first_node: "Node(1)".to_string(),
//                 second_node: "Node(2)".to_string(),
//                 edge: "Edge(Node(1), Node(2))".to_string(),
//             },
//             GraphStructure {
//                 first_node: "Node(3)".to_string(),
//                 second_node: "Node(4)".to_string(),
//                 edge: "Edge(Node(3), Node(4))".to_string(),
//             },
//         ];
//         assert_eq!(deserialized_gr[0].edge, control_content[0].edge);
//         assert_eq!(deserialized_gr[1].edge, control_content[1].edge);
//         assert_ne!(deserialized_gr[0].edge, control_content[1].edge);

//         assert_eq!(deserialized_gr[0].first_node, control_content[0].first_node);
//         assert_eq!(
//             deserialized_gr[0].second_node,
//             control_content[0].second_node
//         );
//         assert_eq!(deserialized_gr[1].first_node, control_content[1].first_node);
//         assert_eq!(
//             deserialized_gr[1].second_node,
//             control_content[1].second_node
//         );
//         assert_ne!(deserialized_gr[0].first_node, control_content[1].first_node);
//     }

//     // 5. BREADTH FIRST SEARCH
//     #[test]
//     fn search_graph_of_char() {
//         let graph_of_char = Graph::new(
//             vec![Node('a'), Node('b'), Node('c'), Node('d')],
//             vec![Edge(Node('a'), Node('b')), Edge(Node('c'), Node('d'))],
//         );
//         let found = bfs(&graph_of_char, Node('b'));
//         assert_eq!(Node('b'), graph_of_char.nodes[1]);
//     }
// }
