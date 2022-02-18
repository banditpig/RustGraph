use crate::graph::{Edge, Node};
use crate::Graph;
use rand::Rng;

use regex::Regex;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn create_random_graph<T: Default, E: Default, ID: Copy + Clone + Hash + Eq>(
    min_nodes: i32,
    max_nodes: i32,
    min_weight: i32,
    max_weight: i32,
) -> Graph<T, i32, i32> {
    //
    let mut g: Graph<T, i32, i32> = Graph::new();
    let mut rng = rand::thread_rng();

    let nodes = rng.gen_range(min_nodes..max_nodes);
    for x in 0..nodes - 1 {
        g.add_node(x, T::default());
    }
    let edges = 2 * nodes; //rng.gen_range(min_nodes..max_nodes);
    for ix in 0..edges {
        let mut l = rng.gen_range(0..nodes - 1);
        let mut r = rng.gen_range(0..nodes - 1);
        while l == r {
            l = rng.gen_range(0..nodes - 1);
            r = rng.gen_range(0..nodes - 1);
        }
        let w = rng.gen_range(min_weight..max_weight);

        if !g.connected(&l, &r) {
            let _ = g.add_edge(ix, l, r, w);
        }
    }

    g
}
pub fn from_viz_dot<T: Default, E: Default, ID: Debug + Copy + Clone + Hash + Eq>(
    path: &str,
) -> Graph<i32, i32, i32> {
    //
    let reader = BufReader::new(File::open(path).expect("Cannot open file"));
    let mut g: Graph<i32, i32, i32> = Graph::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let re = Regex::new(r"^\s*(\d*) -- (\d*) \[label=(\d*), id=(\d*)];").unwrap();
        let matches = re.captures_iter(line.as_str());
        for cap in matches {
            let from: &i32 = &cap[1].to_string().trim().parse().unwrap();
            let to: &i32 = &cap[2].to_string().trim().parse().unwrap();
            let edge_weight: &i32 = &cap[3].to_string().trim().parse().unwrap();
            let edge_id: &i32 = &cap[4].to_string().trim().parse().unwrap();

            g.add_node(*from, 0);
            g.add_node(*to, 0);
            let _ = g.add_edge(*edge_id, *from, *to, *edge_weight);
        }
    }

    g
}

pub fn to_viz_dot<T: Default + Debug, E: Debug, ID: Debug + Copy + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
    path: &str,
) {
    let path = Path::new(path);
    let mut file = File::create(&path).unwrap();
    let mut out_data = String::new();
    out_data.push_str("graph D {");
    out_data.push('\n');

    for Node {
        id,
        data,
        edges: _edges,
    } in g.nodes.values()
    {
        let l = format!("    {:?}[label=\"{:?} \\nData {:?}\"];\r\n", id, id, data);
        out_data.push_str(&l);
    }
    out_data.push('\n');
    for Edge {
        id,
        data,
        left: l,
        right: r,
    } in g.edges.values()
    {
        let l = format!(
            "    {:?} -- {:?} [label={:?}, id={:?}];\r\n",
            l, r, data, id
        );
        out_data.push_str(&l);
    }
    out_data.push('}');
    out_data = out_data.replace("'", "");
    let _ = file.write_all(out_data.as_bytes());
    let _ = file.flush();
}
