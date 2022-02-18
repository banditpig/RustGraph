use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T, ID: Clone + Hash + Eq> {
    pub id: ID,
    pub data: T,
    pub edges: Vec<ID>,
}
#[derive(Debug)]
pub struct Edge<E, ID: Clone + Hash + Eq> {
    pub id: ID,
    pub data: E,
    pub left: ID,
    pub right: ID,
}
impl<E, ID: Clone + Hash + Eq> Edge<E, ID> {
    pub fn new(id: ID, data: E, l: ID, r: ID) -> Self {
        Self {
            id,
            data,
            left: l,
            right: r,
        }
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
    pub nodes: HashMap<ID, Node<T, ID>>,
    pub edges: HashMap<ID, Edge<E, ID>>,
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
    fn start(pos: ID) -> Rc<Self> {
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
        self.nodes.entry(id).or_insert_with(|| Node::new(id, data));
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
        for Edge {
            id: _,
            data: _,
            left: l,
            right: r,
        } in self.edges.values()
        {
            if l == first && r == second || r == first && l == second {
                return true;
            }
        }
        false
    }
    pub fn bfs_path(&self, s: &ID, e: &ID) -> Result<Vec<ID>, GraphError> {
        if !self.nodes.contains_key(s) {
            return Err(GraphError::new("'start' not in nodes."));
        }
        if !self.nodes.contains_key(e) {
            return Err(GraphError::new("'end' not in nodes."));
        }
        let prev = self.solve(s);
        let path = self.reconstruct_path(s, e, prev);

        return path;
    }
    fn solve(&self, p: &ID) -> HashMap<ID, ID> {
        let mut q: VecDeque<ID> = VecDeque::new();
        let mut visited: HashSet<ID> = HashSet::new();
        let mut path: HashMap<ID, ID> = HashMap::new();

        q.push_back(*p);
        visited.insert(*p);

        while !q.is_empty() {
            //we know its not empty
            let node = q.pop_front().unwrap();
            //we know its on the graph
            let neighbours = self.neighbors(node).unwrap();
            for nxt in neighbours {
                if !visited.contains(&nxt) {
                    q.push_back(nxt);
                    visited.insert(nxt);
                    path.insert(nxt, node);
                }
            }
        }

        path
    }
    fn reconstruct_path(
        &self,
        start: &ID,
        end: &ID,
        links: HashMap<ID, ID>,
    ) -> Result<Vec<ID>, GraphError> {
        if !links.contains_key(end) {
            return Err(GraphError::new("End point can't be reqched."));
        }
        let mut path: Vec<ID> = Vec::new();

        path.push(*end);
        let mut next_key = links.get(end).unwrap();
        loop {
            path.push(*next_key);
            if next_key == start {
                path.reverse();
                return Ok(path);
            }
            next_key = links.get(next_key).unwrap();
        }
    }
    pub fn dfs(&self, node: &ID) -> HashSet<ID> {
        let mut v: HashSet<ID> = HashSet::new();
        self.depth_first(node, &mut v);
        return v;
    }
    pub fn depth_first(&self, node: &ID, visited: &mut HashSet<ID>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(*node);
        let neighs = self.neighbors(*node).unwrap();

        for n in neighs {
            self.depth_first(&n, visited);
        }
    }
    pub fn apply_to_nodes(&mut self, f: fn(&mut T) -> T) {
        for mut n in self.nodes.values_mut() {
            let d = n.data.borrow_mut();
            let d = f(d);
            n.data = d;
        }
    }
    pub fn apply_to_edges(&mut self, f: fn(&mut E) -> E) {
        for mut e in self.edges.values_mut() {
            let d = e.data.borrow_mut();
            let d = f(d);
            e.data = d;
        }
    }
    pub fn neighbors(&self, id: ID) -> Result<HashSet<ID>, GraphError> {
        let mut neighs: HashSet<ID> = HashSet::new();

        return match self.nodes.get(&id) {
            None => Err(GraphError::new("'Supplied node id is not in graph.")),
            Some(node) => {
                for edge_id in node.edges.iter() {
                    let Edge {
                        id: _,
                        data: _,
                        left: l,
                        right: r,
                    } = self.edges.get(edge_id).unwrap();
                    neighs.insert(*l);
                    neighs.insert(*r);
                }
                neighs.remove(&id);
                Ok(neighs)
            }
        };
    }
}
