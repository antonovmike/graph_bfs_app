# graph_library

One project: library and binary (not example)

- Add and remove nodes;
- Add and remove directed edges;
- Breadth-first search (or any other algorithm);
- Serialization/deserialization into trivial Graph Format;
- The value type of the graph have to be generic;

This library can:
- Create a new graph Graph::new(some_nodes, some_edges)
- Add and remove nodes add_node(some_graph, some_node)
- Add and remove directed edges add_edge(some_graph, some_edge)
- Serialize/Deserialize graph serial_triv(&some_graph) *
- Breadth first search by Node() bfs(&some_graph, some_node)

* Structures of types String is used. Have to implement a deserializer for generic types 