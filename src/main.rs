use crate::graph::{Graph, GraphError};

mod algorithms;
mod graph;

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

    Ok(())
}
