// iteration 2 inspired by BrightOne's solution using iter().map() to create HashMaps
pub mod graph {
    use std::collections::HashMap;
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }
            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        from,
                        to,
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs = attrs.iter().map(|&(k, v)| (k, v)).collect();
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
            pub struct Node<'a> {
                pub name: &'a str,
                pub attrs: HashMap<&'a str, &'a str>,
            }
            impl<'a> Node<'a> {
                pub fn new(name: &str) -> Node {
                    Node {
                        name,
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs = attrs.iter().map(|&(k, v)| (k, v)).collect();
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| &**v)
                }
            }
        }
    }

    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<&'a str, &'a str>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Graph<'a> {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node<'a>]) -> Graph<'a> {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge<'a>]) -> Graph<'a> {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            self.attrs = attrs.iter().map(|&(k, v)| (k, v)).collect();
            self
        }
        pub fn node(&self, name: &str) -> Option<graph_items::node::Node> {
            self.nodes.iter().find(|&node| node.name == name).cloned()
        }
    }
}
