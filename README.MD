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
- Add and remove directed edges add_edge(some_graph, some_edge)
- Verify Nodes and Graphs uniqueness (hashtable or matrix)


[*] Structures of types String is used. Have to implement a deserializer for generic types 

**FIX**
- add_node - add warning in case Graph is already contins Node with this name
- remove_node - if Node removed check edges and remove all the edges containing this Node

**TODO:**

- Serialize/Deserialize graph serial_triv(&some_graph) [*]
- Breadth first search by Node() bfs(&some_graph, some_node)
- Create interface like HashMap/HashSet/Vec
- Serialization/Deserialization TGF using ToString/FromString
- Use doc comments
- User's path to file
- Executable in separate crate


[Graph Data Structure And Algorithms](https://www.geeksforgeeks.org/graph-data-structure-and-algorithms/)