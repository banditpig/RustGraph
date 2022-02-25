#![allow(dead_code)]

use crate::geometry::{Scalar, Vect};
use crate::graph::Graph;
use crate::utils::create_random_graph;
use crate::Shape::LineSegment;
use eframe::egui::epaint::CircleShape;
use eframe::egui::plot::{Line, Plot};
use eframe::egui::{epaint, CentralPanel, Color32, CtxRef, Pos2, Shape, Stroke, Widget};
use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

mod algorithms;
mod geometry;
mod graph;
mod utils;
const SCALE: f64 = 0.015;
const OFFSET_X: f64 = 400.0;
const OFFSET_Y: f64 = 400.0;
const OFFSET_VEC: Vect = Vect {
    x: OFFSET_X,
    y: OFFSET_Y,
    z: 0.0,
};
struct GraphDisplay<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq> {
    pub g: Graph<T, E, ID>,
    pub points_to_display: HashMap<ID, Vect>,
}

impl<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq> GraphDisplay<T, E, ID> {
    fn connections(&self, points: &HashMap<ID, Vect>) -> Vec<(Vect, Vect)> {
        algorithms::connections(&points, &self.g)
    }
    fn convert_to_line(&self, (s1, e1): &(Vect, Vect)) -> Shape {
        let s = s1.scalar_mul(SCALE) + OFFSET_VEC;
        let e = e1.scalar_mul(SCALE) + OFFSET_VEC;

        LineSegment {
            points: [
                Pos2::new(s.x as f32, s.y as f32),
                Pos2::new(e.x as f32, e.y as f32),
            ],
            stroke: Stroke {
                width: 0.1,
                color: Color32::WHITE,
            },
        }
    }
    fn convert_vect_to_circle(v1: &Vect) -> Shape {
        let v = v1.scalar_mul(SCALE) + OFFSET_VEC;
        let c = Shape::Circle(CircleShape {
            center: Pos2::new(v.x as f32, v.y as f32),
            radius: 5.0,
            fill: Color32::WHITE,
            stroke: Default::default(),
        });
        c
    }
}

impl<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq> App for GraphDisplay<T, E, ID> {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // let c = Shape::Circle(CircleShape {
            //     center: Pos2::new(500., 500.),
            //     radius: 5.0,
            //     fill: Color32::WHITE,
            //     stroke: Default::default(),
            // });
            // let vecs = self.layout();
            for vec in self.points_to_display.values() {
                let c = GraphDisplay::<T, E, ID>::convert_vect_to_circle(&vec);
                ui.painter().add(c);
            }
            for l in self.connections(&self.points_to_display) {
                ui.painter().add(self.convert_to_line(&l));
            }
        });
    }

    fn name(&self) -> &str {
        "GraphDisplay"
    }
}
fn test_graph() -> Graph<i32, i32, i32> {
    let mut g: Graph<i32, i32, i32> = Graph::new();
    for n in 1..23 {
        g.add_node(n, 0)
    }

    g.add_edge(1, 1, 2, 0);
    g.add_edge(2, 11, 3, 0);
    g.add_edge(3, 15, 11, 0);
    g.add_edge(4, 15, 1, 0);
    g.add_edge(5, 15, 19, 0);
    g.add_edge(6, 16, 1, 0);
    g.add_edge(7, 16, 14, 0);
    g.add_edge(8, 17, 16, 0);
    // g.add_edge(9,,0);
    g.add_edge(10, 17, 19, 0);
    g.add_edge(11, 19, 22, 0);
    g.add_edge(12, 19, 18, 0);
    g.add_edge(13, 17, 18, 0);
    g.add_edge(14, 17, 20, 0);
    g.add_edge(15, 18, 22, 0);
    g.add_edge(16, 18, 20, 0);
    g.add_edge(17, 20, 21, 0);
    g.add_edge(18, 20, 13, 0);
    g.add_edge(19, 22, 21, 0);
    g.add_edge(20, 21, 7, 0);
    g.add_edge(21, 13, 14, 0);
    g.add_edge(22, 22, 12, 0);
    g.add_edge(23, 12, 6, 0);
    g.add_edge(24, 8, 9, 0);
    g.add_edge(25, 13, 8, 0);
    g.add_edge(26, 7, 8, 0);
    g.add_edge(27, 7, 6, 0);
    g.add_edge(28, 6, 5, 0);
    g.add_edge(29, 10, 9, 0);
    g.add_edge(30, 10, 2, 0);
    g.add_edge(31, 4, 9, 0);
    g.add_edge(32, 3, 5, 0);
    g.add_edge(33, 4, 5, 0);
    g.add_edge(34, 2, 4, 0);
    g.add_edge(35, 2, 3, 0);
    g.add_edge(36, 10, 4, 0);
    g.add_edge(37, 14, 10, 0);
    g.add_edge(38, 11, 12, 0);

    g
}

fn main() {
    //test_graph(); //
    let graph: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(20, 50, 1, 10, 0, 1);
    let p: HashMap<i32, Vect> = algorithms::layout(&graph);
    let app = GraphDisplay {
        g: graph,
        points_to_display: p,
    };
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}
