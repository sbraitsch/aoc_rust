use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct GraphNode<T> {
    id: usize,
    value: T,
    edges: Vec<Edge<T>>,
}

#[derive(Debug)]
pub struct Edge<T> {
    target: Rc<RefCell<GraphNode<T>>>,
    weight: Option<usize>,
}

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Rc<RefCell<GraphNode<T>>>>,
    next_id: usize,
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SearchState {
    cost: usize,
    node_id: usize,
}

impl<T> Graph<T> {

    pub fn new() -> Self {
        Self { nodes: Vec::new(), next_id: 0 }
    }

    pub fn add_node(&mut self, value: T) -> Rc<RefCell<GraphNode<T>>> {
        let node = Rc::new(RefCell::new(GraphNode { id: self.next_id, value, edges: Vec::new() }));
        self.next_id += 1;
        self.nodes.push(node.clone());
        node
    }

    pub fn add_edge(
        &mut self,
        from: &Rc<RefCell<GraphNode<T>>>,
        to: &Rc<RefCell<GraphNode<T>>>,
        weight: Option<usize>,
    ) {
        let edge = Edge {
            target: Rc::clone(to),
            weight,
        };
        from.borrow_mut().edges.push(edge);
    }

    pub fn shortest_path(
        &self,
        start: &Rc<RefCell<GraphNode<T>>>,
        target: &Rc<RefCell<GraphNode<T>>>,
    ) -> Option<(usize, Vec<Rc<RefCell<GraphNode<T>>>>)>
    where
        T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
    {
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let mut previous: HashMap<usize, Option<usize>> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for node in &self.nodes {
            let node_id = node.borrow().id;
            distances.insert(node_id, usize::MAX);
            previous.insert(node_id, None);
        }

        let start_id = start.borrow().id;
        distances.insert(start_id, 0);
        heap.push(SearchState {
            cost: 0,
            node_id: start_id,
        });

        while let Some(SearchState { cost, node_id }) = heap.pop() {
            if node_id == target.borrow().id {
                let mut path = Vec::new();
                let mut current = Some(node_id);
                while let Some(node_id) = current {
                    let node = self
                        .nodes
                        .iter()
                        .find(|n| n.borrow().id == node_id)
                        .expect("Node not found");
                    path.push(Rc::clone(node));
                    current = previous[&node_id];
                }
                path.reverse();
                return Some((cost, path));
            }

            if cost > distances[&node_id] {
                continue;
            }

            let current_node = self
                .nodes
                .iter()
                .find(|n| n.borrow().id == node_id)
                .expect("Node not found");

            for edge in &current_node.borrow().edges {
                let next = edge.target.borrow().id;
                let weight = edge.weight.unwrap_or(1);
                let next_cost = cost + weight;

                if next_cost < distances[&next] {
                    distances.insert(next, next_cost);
                    previous.insert(next, Some(node_id));
                    heap.push(SearchState {
                        cost: next_cost,
                        node_id: next,
                    });
                }
            }
        }

        None
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

