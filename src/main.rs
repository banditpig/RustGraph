#![allow(dead_code)]

use crate::geometry::Vect;
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

struct GraphDisplay<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq> {
    pub g: Graph<T, E, ID>,
}

impl<T, E, ID: Debug + Copy + Ord + Clone + Hash + Eq> GraphDisplay<T, E, ID> {
    fn layout(&self) -> HashMap<ID, Vect> {
        algorithms::layout(&self.g)
    }
    fn connections(&self, points: HashMap<ID, Vect>) -> Vec<(Vect, Vect)> {
        algorithms::connections(&points, &self.g)
    }
    fn convert_to_line(&self, (s, e): &(Vect, Vect)) -> Shape {
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
    fn convert_vect_to_circle(v: &Vect) -> Shape {
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
            let vecs = self.layout();
            for vec in vecs.values() {
                let c = GraphDisplay::<T, E, ID>::convert_vect_to_circle(&vec);
                ui.painter().add(c);
            }
            for l in self.connections(vecs) {
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
    let graph: Graph<i32, i32, i32> = test_graph(); //create_random_graph::<i32, i32, i32>(10, 31, 1, 10, 0, 1);

    let app = GraphDisplay { g: graph };
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}
