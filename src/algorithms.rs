use crate::geometry::{Scalar, Vect};

use crate::Graph;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

const REPEL_CONST: Scalar = 2.0;
const SPRING_CONST: Scalar = 1.0;
const SPRING_LENGTH: Scalar = 5.0;
const DELTA_T: Scalar = 0.1;

pub fn layout<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
) -> HashMap<ID, Vect> {
    let mut positions = create_initial_positions(g);
    println!("{:?}", positions);

    for _i in 1..100 {
        let repel_forces = calculate_repel_forces(g, &mut positions);
        let spring_forces = calculate_spring_forces(g, &mut positions);
        let resultant_forces = calculate_resultant_forces(repel_forces, spring_forces);
        update_positions(&mut positions, resultant_forces);
    }
    println!("=> {:?}", positions);
    positions
}
fn repelling_force(pos_u: &Vect, pos_v: &Vect) -> Vect {
    //applies to node u and ALL other nodes
    //unit v in direction of u -> v   v-u
    let unit_uv = (*pos_u - *pos_v).as_unit_vector();
    let euc_dist = pos_u.euclid_distance(pos_v);
    unit_uv.scalar_mul(REPEL_CONST / euc_dist * euc_dist)
}
fn spring_force(pos_u: &Vect, pos_v: &Vect) -> Vect {
    //applies to node u and all its immediate neighbours.
    let unit_vu = (*pos_v - *pos_u).as_unit_vector();
    let euc_dist = pos_u.euclid_distance(pos_v);
    let x = SPRING_CONST * (euc_dist.powf(2.0) / SPRING_LENGTH).ln();
    unit_vu.scalar_mul(x)
}
fn create_initial_positions<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
) -> HashMap<ID, Vect> {
    let mut positions: HashMap<ID, Vect> = HashMap::new();
    for node_id in g.nodes.keys() {
        //setup with initial vect.
        positions.insert(*node_id, Vect::random(0., 800.0, false));
    }
    positions
}

fn update_positions<ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    positions: &mut HashMap<ID, Vect>,
    resultant_forces: HashMap<ID, Vect>,
) -> HashMap<ID, Vect> {
    let mut new_positions: HashMap<ID, Vect> = HashMap::new();
    for pos in positions.keys() {
        let old_pos = positions.get(pos).unwrap();
        let f = resultant_forces.get(pos).unwrap().scalar_mul(DELTA_T);
        let new_pos = *old_pos + f;
        new_positions.insert(*pos, new_pos);
    }
    new_positions
}

fn calculate_resultant_forces<ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    repel_forces: HashMap<ID, Vect>,
    spring_forces: HashMap<ID, Vect>,
) -> HashMap<ID, Vect> {
    let mut resultant_forces: HashMap<ID, Vect> = HashMap::new();
    for u in repel_forces.keys() {
        let temp_repel = repel_forces.get(u).unwrap();
        let temp_spring = spring_forces.get(u).unwrap();
        resultant_forces.insert(*u, *temp_repel + *temp_spring);
    }
    resultant_forces
}

fn calculate_spring_forces<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
    positions: &mut HashMap<ID, Vect>,
) -> HashMap<ID, Vect> {
    let mut spring_forces: HashMap<ID, Vect> = HashMap::new();
    for node_u in g.nodes.keys().sorted() {
        let mut spring_u = Vect::new(0., 0., 0.);
        let u = positions.get(node_u).unwrap();
        for node_v in g.neighbors(*node_u).unwrap() {
            let v = positions.get(&node_v).unwrap();
            spring_u = spring_u + spring_force(u, v) - repelling_force(u, v)
        }
        spring_forces.insert(*node_u, spring_u);
    }
    spring_forces
}

fn calculate_repel_forces<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq>(
    g: &Graph<T, E, ID>,
    positions: &mut HashMap<ID, Vect>,
) -> HashMap<ID, Vect> {
    //
    let mut repel_forces: HashMap<ID, Vect> = HashMap::new();
    for node_u in g.nodes.keys().sorted() {
        let u = positions.get(node_u).unwrap();
        let mut repel_u = Vect::new(0., 0., 0.);
        for node_v in g.nodes.keys().sorted() {
            if node_u != node_v {
                let v = positions.get(node_v).unwrap();
                repel_u = repel_u + repelling_force(u, v);
            }
        }
        repel_forces.insert(*node_u, repel_u);
    }
    repel_forces
}
#[cfg(test)]
use crate::utils::create_random_graph;
#[test]
pub fn x() {
    let g: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(50, 100, 1, 10, 0, 1);
    println!("{:?}", layout(&g));
}
