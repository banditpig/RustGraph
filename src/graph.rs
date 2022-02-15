use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T, ID: Clone + Hash + Eq> {
    id: ID,
    data: T,
    edges: Vec<ID>,
}
#[derive(Debug)]
pub struct Edge<E, ID: Clone + Hash + Eq> {
    id: ID,
    data: E,
    l: ID,
    r: ID,
}
impl<E, ID: Clone + Hash + Eq> Edge<E, ID> {
    pub fn new(id: ID, data: E, l: ID, r: ID) -> Self {
        Self { id, data, l, r }
    }
}

impl<T, ID: Clone + Hash + Eq> Node<T, ID> {
    pub fn new(id: ID, data: T) -> Self {
        Self {
            data,
            id,
            edges: Vec::new(),
        }
    }
}
#[derive(Debug)]
pub struct Graph<T, E, ID: Clone + Hash + Eq> {
    nodes: HashMap<ID, Node<T, ID>>,
    edges: HashMap<ID, Edge<E, ID>>,
}

#[derive(Debug)]
pub struct GraphError {
    message: String,
}

impl GraphError {
    fn new(m: &str) -> Self {
        Self {
            message: m.to_string(),
        }
    }
}

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}
#[derive(Debug)]
struct Route<ID> {
    position: ID,
    path: Option<Rc<Route<ID>>>,
    length: i32,
}

impl<ID: fmt::Debug> Display for Route<ID> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(ref p) = self.path {
            write!(f, "{}-{}-", p, self.length)?;
        }
        let _ = write!(f, "{:?}", self.position);
        Ok(())
    }
}
impl<ID: Eq> Route<ID> {
    fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            position: pos,
            path: None,
            length: 0,
        })
    }
    fn contains(&self, id: &ID) -> bool {
        if self.position == *id {
            return true;
        }
        match &self.path {
            None => false,
            Some(ref p) => p.contains(id),
        }
    }
}
impl<T, E, ID: Copy + Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        self.nodes.insert(id, Node::new(id, data));
    }
    pub fn add_edge(
        &mut self,
        edge_id: ID,
        from: ID,
        to: ID,
        edge_data: E,
    ) -> Result<(), GraphError> {
        //
        if !self.nodes.contains_key(&from) {
            return Err(GraphError::new("'from' not in nodes."));
        }
        if !self.nodes.contains_key(&to) {
            return Err(GraphError::new("'to' not in nodes."));
        }

        let edge = Edge::new(edge_id, edge_data, from, to);
        self.edges.insert(edge_id.clone(), edge);

        let n = self.nodes.get_mut(&from.clone()).unwrap();
        n.edges.push(edge_id);

        let n = self.nodes.get_mut(&to.clone()).unwrap();
        n.edges.push(edge_id);

        Ok(())
    }
    pub fn connected(&self, first: &ID, second: &ID) -> bool {
        for Edge { id, data, l, r } in self.edges.values() {
            if l == first && r == second || r == first && l == second {
                return true;
            }
        }
        false
    }

    pub fn neighbors(&self, id: ID) -> Result<HashSet<ID>, GraphError> {
        let mut neighs: HashSet<ID> = HashSet::new();

        return match self.nodes.get(&id) {
            None => Err(GraphError::new("'Supplied node id is not in graph.")),
            Some(node) => {
                for edge_id in node.edges.iter() {
                    let Edge { id, data, l, r } = self.edges.get(edge_id).unwrap();
                    neighs.insert(*l);
                    neighs.insert(*r);
                }
                neighs.remove(&id);
                Ok(neighs)
            }
        };
    }
}
