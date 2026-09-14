pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            /// An edge between two nodes in the graph, with optional attributes.
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                /// Constructs a new `Edge` between two node identifiers.
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                /// Adds attributes to the edge using a builder pattern.
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                /// Retrieves the value of an attribute by key if it exists.
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }

                /// Alias for `attr` to ensure API compatibility.
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attr(key)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            /// A node in the graph with an identifier and optional attributes.
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                /// Constructs a new `Node` with the specified name.
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                /// Adds attributes to the node using a builder pattern.
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                /// Retrieves the value of an attribute by key if it exists.
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }

                /// Alias for `attr` to ensure API compatibility.
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attr(key)
                }
            }
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    /// Graph representation composed of nodes, edges, and graph-level attributes.
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        /// Creates a new empty `Graph`.
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        /// Configures the graph with a list of nodes using a builder pattern.
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        /// Configures the graph with a list of edges using a builder pattern.
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        /// Configures graph-level attributes using a builder pattern.
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for &(k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }

        /// Finds a node by name if present.
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }

        /// Alias for `node` to ensure API compatibility.
        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.node(name)
        }

        /// Retrieves the value of a graph attribute by key.
        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|v| v.as_str())
        }

        /// Alias for `attr` to ensure API compatibility.
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attr(key)
        }
    }
}
