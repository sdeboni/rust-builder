// This version with cloning is 2x slower thatn the version
// based on passing and maintaining references only
pub mod graph {
    use std::collections::HashMap;
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs.iter() {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| &**v)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr in attrs.iter() {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| &**v)
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Graph {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Graph {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs.iter() {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }
        pub fn node(&self, name: &str) -> Option<graph_items::node::Node> {
            self.nodes.iter().find(|&node| node.name == name).cloned()
        }
    }
}
