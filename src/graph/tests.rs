use crate::graph::Graph;
use crate::utils::create_random_graph;
use crate::GraphError;
use rand::Rng;

#[test]
fn basic_connections() {
    let mut g: Graph<usize, usize, char> = Graph::new();
    for x in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, 0);
    }
    g.add_edge('a', 'H', 'D', 6).unwrap();
    g.add_edge('b', 'D', 'C', 18).unwrap();
    g.add_edge('c', 'C', 'B', 10).unwrap();
    g.add_edge('d', 'H', 'A', 7).unwrap();
    g.add_edge('e', 'A', 'C', 4).unwrap();
    g.add_edge('f', 'H', 'G', 5).unwrap();
    g.add_edge('g', 'G', 'A', 8).unwrap();
    g.add_edge('h', 'A', 'F', 3).unwrap();
    g.add_edge('i', 'F', 'E', 15).unwrap();
    g.add_edge('j', 'C', 'E', 12).unwrap();
    assert_eq!(g.nodes.keys().count(), 8);
    assert_eq!(g.edges.keys().count(), 10);

    assert_eq!(true, g.connected(&'C', &'A'));
    assert_eq!(true, g.connected(&'A', &'C'));
}

#[test]
fn bfs_path() {
    let mut g: Graph<usize, usize, char> = Graph::new();
    for x in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'] {
        g.add_node(x, 0);
    }
    g.add_edge('a', 'H', 'D', 6).unwrap();
    g.add_edge('b', 'D', 'C', 18).unwrap();
    g.add_edge('c', 'C', 'B', 10).unwrap();
    g.add_edge('d', 'H', 'A', 7).unwrap();
    g.add_edge('e', 'A', 'C', 4).unwrap();
    g.add_edge('f', 'H', 'G', 5).unwrap();
    g.add_edge('g', 'G', 'A', 8).unwrap();
    g.add_edge('h', 'A', 'F', 3).unwrap();
    g.add_edge('i', 'F', 'E', 15).unwrap();
    g.add_edge('j', 'C', 'E', 12).unwrap();
    let p = g.bfs_path(&'G', &'B');
    match p {
        Ok(v) => {
            assert_eq!(4, v.len());
            assert!(v.contains(&'G'));
            assert!(v.contains(&'A'));
            assert!(v.contains(&'C'));
            assert!(v.contains(&'B'));
        }
        Err(_) => assert_eq!(false, true),
    }
    let p = g.bfs_path(&'G', &'Z');
    match p {
        Ok(_) => assert_eq!(false, true),
        Err(_) => assert!(true),
    }
}

#[test]
fn apply_nodes() {
    let mut g1: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(10, 20, 1, 10);

    g1.nodes.values().for_each(|n| assert_eq!(0, n.data));

    g1.apply_to_nodes(|_| {
        let mut rng = rand::thread_rng();
        rng.gen_range(10..100)
    });
    g1.nodes
        .values()
        .for_each(|n| assert!(n.data >= 10 && n.data <= 100));
}
#[test]
fn apply_edges() {
    let mut g1: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(10, 20, 1, 10);
    g1.edges
        .values()
        .for_each(|e| assert!(e.data >= 1 && e.data <= 10));

    g1.apply_to_edges(|_| 1234);
    g1.edges.values().for_each(|e| assert!(e.data == 1234));
}

// to_viz_dot(&g1, "test.dot");
//
// let g2 = from_viz_dot::<i32, i32, i32>("test.dot");
