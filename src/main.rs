use std::collections::{HashMap, HashSet};

use eframe::{egui::{
    self, pos2, Color32, Label, Sense, Shape, Stroke, Vec2 
}, epaint::TextShape};

const ISLAND_SIZE: f32 = 50.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
struct Island {
    required_bridges: usize,
    connected_bridges: usize,
}

#[derive(Default)]
struct HashiBoard {
    islands: HashMap<Position, Island>,
    bridges: HashMap<Position, HashSet<Position>>,
}

impl HashiBoard {
    fn add_island(&mut self, position: Position, required_bridges: usize) {
        todo!();
    }

    fn add_bridge(&mut self, pos1: Position, pos2: Position) {
        todo!();
    }
}

impl eframe::App for HashiBoard { 
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

            let mut control_points =  [
                pos2(50.0, 50.0),
                pos2(100.0, 100.0),
                pos2(50.0, 100.0),
            ];
            let control_point_radius = 8.0;

            let control_point_shapes: Vec<Shape> = 
            control_points
            .iter_mut()
            .map(| point| {

                let label = Label::new("1").sense(Sense::click());
                let (_, galley, _) = label.layout_in_ui(ui);
                let text_pos = *point - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);

                vec![
                    Shape::circle_stroke(*point, control_point_radius, Stroke::new(1.0, Color32::BLACK)),
                    Shape::Text(TextShape::new(text_pos, galley, Color32::BLACK))
                ]
            })
            .flatten()
            .collect();

            painter.extend(control_point_shapes);

            response
        });
    }
}

fn main() -> eframe::Result {
    let hashi_board = HashiBoard {
        islands: HashMap::from([
            (
                Position { x: 0, y: 0 },
                Island {
                    required_bridges: 2,
                    connected_bridges: 0,
                },
            ),
            (
                Position { x: 2, y: 0 },
                Island {
                    required_bridges: 1,
                    connected_bridges: 0,
                },
            ),
            (
                Position { x: 0, y: 2 },
                Island {
                    required_bridges: 1,
                    connected_bridges: 0,
                },
            ),
        ]),
        bridges: HashMap::new(),
    };

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::from(hashi_board))),
    )
}
