use serde::{self, ser::SerializeStruct, ser::SerializeTuple};
use std::rc::Rc;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Rc<GraphNode>>,
    edges: Vec<GraphEdge>,
}

impl serde::Serialize for Graph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Graph", 2)?;
        let nodes: Vec<&GraphNode> = self.nodes.iter().map(|node| node.as_ref()).collect();
        state.serialize_field("nodes", &nodes)?;
        state.serialize_field("edges", &self.edges)?;
        state.end()
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn find_by_id(&self, id: u32) -> Option<&Rc<GraphNode>> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn add_node(&mut self, id: u32, word: &str, adjacent_nodes: Vec<u32>) {
        let new_node = Rc::new(GraphNode {
            id,
            word: String::from(word),
        });

        self.nodes.push(Rc::clone(&new_node));

        let adjacent_nodes: Vec<Rc<GraphNode>> = adjacent_nodes
            .iter()
            .filter_map(|id| self.find_by_id(*id))
            .map(|node| Rc::clone(node))
            .collect();

        for node in adjacent_nodes {
            self.edges.push(GraphEdge {
                nodes: (Rc::clone(&new_node), node),
            });
        }
    }

    pub fn delete_node(&mut self, id: u32) {
        let Some(node_to_delete) = self.nodes.iter().find(|node| node.id == id) else {
            return;
        };

        let node_to_delete = Rc::clone(node_to_delete);

        if let Some(index) = self.nodes.iter().position(|node| node.id == id) {
            self.nodes.remove(index);
        }

        self.edges.retain(|edge| {
            let (node1, node2) = &edge.nodes;
            node1.id != id && node2.id != id
        });

        drop(node_to_delete);
    }

    pub fn add_edge(&mut self, id1: u32, id2: u32) {
        let (Some(node1), Some(node2)) = (self.find_by_id(id1), self.find_by_id(id2)) else {
            return;
        };
        self.edges.push(GraphEdge {
            nodes: (Rc::clone(node1), Rc::clone(node2)),
        });
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GraphNode {
    id: u32,
    word: String,
}

#[derive(Debug)]
struct GraphEdge {
    nodes: (Rc<GraphNode>, Rc<GraphNode>),
}

impl serde::Serialize for GraphEdge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.nodes.0.id);
        tuple.serialize_element(&self.nodes.1.id);
        tuple.end()
    }
}
