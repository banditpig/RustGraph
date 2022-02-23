#![allow(dead_code)]

use crate::geometry::Vect;
use crate::graph::Graph;
use crate::utils::create_random_graph;
use eframe::egui::epaint::CircleShape;
use eframe::egui::plot::Plot;
use eframe::egui::{CentralPanel, Color32, CtxRef, Pos2, Shape, Widget};
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

            ui.heading("Hello World!");
        });
    }

    fn name(&self) -> &str {
        "GraphDisplay"
    }
}

fn main() {
    let graph: Graph<i32, i32, i32> = create_random_graph::<i32, i32, i32>(50, 100, 1, 10, 0, 1);

    let app = GraphDisplay { g: graph };
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
}
