#![allow(unused)]
use std::collections::{HashSet, VecDeque};

type NodeType = Vec<i32>;
type GraphType = Vec<NodeType>;
struct Queue<T> {
    pub items: VecDeque<T>,
}
impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push_back(v)
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from empty queue.")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}

#[derive(Clone)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<Edge>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node<T>(pub T);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Edge(pub i32, pub i32);

impl<T> Graph<T> {
    pub fn new(nodes: Vec<Node<T>>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }
}

impl<T> From<T> for Node<T> {
    fn from(item: T) -> Self {
        Node(item)
    }
}

impl<T> Node<T> {
    pub fn value(&self) -> T 
    where T: Copy{
        self.0
    }

    pub fn neighbors(&self, graph: &Graph<T>) -> Vec<Node<T>> 
    where T: PartialEq + Copy
    {
        graph
            .nodes
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.0.into())
            .collect()
    }
}

// --> ADD AND REMOVE NODES

pub fn add_node<T>(graph: Graph<T>, to_add: Node<T>) -> Graph<T> {
    let mut new_vec = graph;
    new_vec.nodes.push(to_add);
    new_vec
}

pub fn rem_node<T>(graph: Graph<T>, to_remove: Node<T>) -> Graph<T> 
where T: PartialEq
{
    let mut nodes = graph.nodes;
    nodes.retain(|value: &Node<T> | *value != to_remove);
    let new_vec: Graph<T> = Graph {
        nodes: nodes,
        edges: graph.edges,
    };
    new_vec
}

// --> ADD AND REMOVE EDGES

pub fn add_edge<T>(graph: Graph<T>, to_add: Edge) -> Graph<T> {
    let mut new_vec = graph;
    new_vec.edges.push(to_add);
    new_vec
}

pub fn rem_edge<T>(graph: Graph<T>, to_remove: Edge) -> Graph<T> {
    let mut edges: Vec<Edge> = graph.edges;
    edges.retain(|value: &Edge | *value != to_remove);
    let new_vec = Graph {
        nodes: graph.nodes,
        edges: edges,
    };
    new_vec
}

// SERDE INTO TRIVIAL GRAPH FORMAT

// BREADTH FIRST SEARCH
// Use a list that stores nodes that need to be browsed.
// In one iteration of the algorythm:
// - if the list is not empty, the node is extracted from the list
// - the extracted node is visited (processed)
// - all of the children are placed into the list
// pub fn bfs(graph: GraphType, start_node: i32, end_node: i32) -> Option<Vec<Option<i32>>> {
//     let mut queue = Queue::new();
//     queue.enqueue(start_node);

//     let mut visisted_nodes = vec![false; graph.len()];
//     visisted_nodes[0] = true;

//     let mut prev: Vec<Option<i32>> = vec![None; graph.len()];

//     'outer: while !queue.is_empty() {
//         let current_node = queue.dequeue();
//         for v in graph[current_node as usize].iter() {
//             if *v == end_node {
//                 prev[*v as usize] = Some(current_node);
//                 break 'outer;
//             }

//             if !visisted_nodes[*v as usize] {
//                 queue.enqueue(*v);
//                 visisted_nodes[*v as usize] = true;
//                 prev[*v as usize] = Some(current_node);
//             }
//         }
//     }

//     let mut path = Vec::new();
//     let mut at = Some(end_node);
//     while at != None {
//         path.push(at);
//         at = prev[at.unwrap_or(0) as usize];
//     }

//     path.reverse();
    
//     return match path[0] {
//         Some(x) if x == start_node => Some(path),
//         _ => None,
//     };
// }

fn main() {}

// pub fn bfs(graph: &Graph, root: Node, target: Node) -> Option<Vec<i32>> {
//     println!("root {:?} target {:?}", root, target);
//     let mut visited: HashSet<Node> = HashSet::new();
//     let mut history: Vec<i32> = Vec::new();
//     let mut queue = VecDeque::new();

//     visited.insert(root);
//     queue.push_back(root);
//     while let Some(currentnode) = queue.pop_front() {
//         history.push(currentnode.value());

//         if currentnode == target {
//             println!("Goal is found: {:?}", history);
//             return Some(history);
//         }

//         for neighbor in currentnode.neighbors(graph) {
//             if !visited.contains(&neighbor) {
//                 visited.insert(neighbor);
//                 queue.push_back(neighbor);
//             }
//         }
//     }

//     None
// }