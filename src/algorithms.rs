use crate::geometry::{Scalar, Vect};

use crate::utils::create_random_graph;
use crate::Graph;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

const REPEL_CONST: Scalar = 2.0;
const SPRING_CONST: Scalar = 1.0;
const SPRING_LENGTH: Scalar = 1.0;

pub fn repel_force(pos_u: &Vect, pos_v: &Vect) -> Vect {
    //applies to node u and ALL other nodes
    let unit_uv = (*pos_v - *pos_u).as_unit_vector();
    unit_uv.scalar_mul(2.0 / pos_u.euclid_distance(pos_v))
}
pub fn spring_force(pos_u: &Vect, pos_v: &Vect) -> Vect {
    //applies to node u and all its immediate neighbours.
    let unit_vu = (*pos_u - *pos_v).as_unit_vector();
    let x = SPRING_CONST * (pos_u.euclid_distance(pos_v) / SPRING_LENGTH).ln();
    unit_vu.scalar_mul(x)
}
pub fn layout<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
) -> HashMap<ID, Vect> {
    let mut p: HashMap<ID, Vect> = HashMap::new();
    for node_id in g.nodes.keys() {
        //setup with initial vect.
        p.insert(*node_id, Vect::random(0.0, 100.0, false));
    }
    //all possible pairs of ids
    let pairs = uv_pairs(g);
    for (u, v) in pairs.iter() {
        let pu = p.get(u).unwrap();
        let pv = p.get(v).unwrap();
        println!("Repel {:?} {:?} {:?}", u, v, repel_force(pu, pv));
    }
    for u in g.nodes.keys() {
        //attraction between node and its neighbours
        let neighbours = g.neighbors(*u).unwrap();
        for neigh_id in neighbours {
            let pu = p.get(u).unwrap();

            let pv = p.get(&neigh_id).unwrap();
            let f = spring_force(pu, pv);
            println!("Attract {:?} {:?} {:?}", u, neigh_id, f);
        }
    }

    p
}

fn uv_pairs<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
) -> Vec<(ID, ID)> {
    let mut id_pairs: Vec<(ID, ID)> = Vec::new();
    let mut v_ids: Vec<ID> = Vec::new();
    //doesn't need to be sorted
    for id in g.nodes.keys().sorted() {
        v_ids.push(*id)
    }
    for (ix, id) in v_ids.iter().enumerate() {
        for index in ix..v_ids.len() {
            let a = *id;
            let b = v_ids[index];
            if a != b {
                id_pairs.push((*id, v_ids[index]))
            }
        }
    }
    id_pairs
}

#[test]
pub fn x() {
    let g: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(10, 11, 1, 10, 0, 1);
    println!("{:?}", layout(&g));
}
