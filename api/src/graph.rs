use std::rc::Rc;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Rc<GraphNode>>,
    edges: Vec<GraphEdge>,
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

    pub fn add_node(&mut self, word: &str, adjacent_nodes: Vec<u32>) {
        let id = match self.nodes.last() {
            Some(node) => node.id + 1,
            None => 1,
        };

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

#[derive(Debug)]
pub struct GraphNode {
    id: u32,
    word: String,
}

#[derive(Debug)]
struct GraphEdge {
    nodes: (Rc<GraphNode>, Rc<GraphNode>),
}
