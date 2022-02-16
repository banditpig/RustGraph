use crate::graph::{Graph, GraphError};
use crate::utils::viz_dot;
use std::collections::HashSet;
use utils::create_random_graph;

mod algorithms;
mod graph;
mod utils;

fn main() -> Result<(), GraphError> {
    let mut g: Graph<usize, usize, char> = Graph::new();
    for x in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, 0);
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('z', 'D', 'H', 6)?;

    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    g.add_edge('h', 'A', 'F', 3)?;
    g.add_edge('i', 'F', 'E', 15)?;
    g.add_edge('j', 'C', 'E', 12)?;
    println!("{:?}", g);

    println!("{:?}", g.neighbors('A'));
    println!("{:?}", g.neighbors('F'));
    println!("{:?}", g.neighbors('C'));
    println!("{:?}", g.neighbors('Z'));

    println!("{:?}", g.connected(&'Z', &'A'));
    println!("{:?}", g.connected(&'C', &'A'));
    println!("{:?}", g.connected(&'A', &'C'));

    println!("{:?}", g.dfs(&'H'));

    //g.bfs_path(&'G', &'B');
    println!("{:?}", g.bfs_path(&'D', &'A'));
    let g2: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(10, 50);
    println!("{:?}", g2);
    viz_dot(g, "test.dot");
    Ok(())
}
