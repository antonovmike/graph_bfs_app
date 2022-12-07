# graph_library

**One project: library and binary (not example)**

- Add and remove nodes;
- Add and remove directed edges;
- Breadth-first search (or any other algorithm);
- Serialization/deserialization into trivial Graph Format;
- The value type of the graph have to be generic;

**This library can:**

- Create nodes from the list
- Create a new graph Graph::new(some_nodes, some_edges)
- Add and remove nodes add_node(some_graph, some_node)
- Create nodes from the list
- Create a new graph Graph::new(some_nodes, some_edges)
- Add and remove nodes add_node(some_graph, some_node)
- Check Node's identity and avoid adding duplicates


[*] Structures of types String is used. Have to implement a deserializer for generic types 

**TODO:**

- Add and remove directed edges add_edge(some_graph, some_edge)
- Serialize/Deserialize graph serial_triv(&some_graph) [*]
- Breadth first search by Node() bfs(&some_graph, some_node)
- Create interface like HashMap/HashSet/Vec
- Verify Nodes and Graphs uniqueness (hashtable or matrix)
For example
```rust
struct Graph<N, E> {
    nodes: HashMap<u64, N>,
    edges: HashMap<u64, HashMap<u64, E>>,
}
```
- Serialization/Deserialization TGF using ToString/FromString
- Use doc comments
- User's path to file
- Executable in separate crate


[Graph Data Structure And Algorithms](https://www.geeksforgeeks.org/graph-data-structure-and-algorithms/)