use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct GraphNode<T> {
    value: T,
    edges: Vec<Edge<T>>,
}

#[derive(Debug)]
pub struct Edge<T> {
    target: Rc<RefCell<GraphNode<T>>>,
    weight: Option<i64>,
}

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Rc<RefCell<GraphNode<T>>>>,
}

impl<T> Graph<T> {

    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: T) -> Rc<RefCell<GraphNode<T>>> {
        let node = Rc::new(RefCell::new(GraphNode { value, edges: Vec::new() }));
        self.nodes.push(node.clone());
        node
    }

    pub fn add_edge(
        &mut self,
        from: &Rc<RefCell<GraphNode<T>>>,
        to: &Rc<RefCell<GraphNode<T>>>,
        weight: Option<i64>,
    ) {
        let edge = Edge {
            target: Rc::clone(to),
            weight,
        };
        from.borrow_mut().edges.push(edge);
    }
}

impl<T: std::fmt::Debug> GraphNode<T> {
    pub fn display_edges(&self) {
        println!("Node ({:?}) has edges to:", self.value);
        for edge in &self.edges {
            println!("  -> {:?}", edge.target.borrow().value);
        }
    }
}
