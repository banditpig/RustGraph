use crate::graph::Edge;
use crate::{graph, Graph, GraphError};
use rand::Rng;

use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::path::Path;
use std::process::id;

pub fn create_random_graph<T: Default, E, ID: Copy + Clone + Hash + Eq>(
    min_nodes: i32,
    max_nodes: i32,
) -> Graph<T, E, i32> {
    let mut g: Graph<T, E, i32> = Graph::new();
    let mut rng = rand::thread_rng();
    let nodes = rng.gen_range(min_nodes..max_nodes);
    for x in 0..nodes - 1 {
        g.add_node(x, T::default());
    }
    g
}

pub fn viz_dot<T: Default, E, ID: Debug + Copy + Clone + Hash + Eq>(
    g: Graph<T, E, ID>,
    path: &str,
) {
    let path = Path::new(path);
    let mut file = File::create(&path).unwrap();
    let mut out_data = String::new();
    out_data.push_str("graph D {");
    out_data.push_str("\n");

    // for n in g.nodes.keys() {
    //     let l = format!("    {:?}", n) + " [shape=circle]\n";
    //     out_data.push_str(&l);
    // }
    for Edge {
        id,
        data,
        left: l,
        right: r,
    } in g.edges.values()
    {
        let l = format!("    {:?} -- {:?}; \n", l, r);
        out_data.push_str(&l);
    }
    out_data.push_str("}");
    out_data = out_data.replace("'", "");
    file.write_all(out_data.as_bytes());
    file.flush();
}
