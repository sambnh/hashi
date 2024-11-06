use std::collections::{HashMap, HashSet};

use eframe::egui::{
    self, text::LayoutJob, Align, Color32, FontFamily, FontId, Pos2, Rounding, TextFormat, Vec2,
};

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
            ui.add(
                egui::Button::new({
                    let mut job = LayoutJob {
                        halign: Align::Center,
                        ..Default::default()
                    };
                    job.append(
                        "1",
                        0.0,
                        TextFormat {
                            font_id: FontId::new(14.0, FontFamily::Proportional),
                            color: Color32::BLACK,
                            ..Default::default()
                        },
                    );
                    job
                })
                .rounding(Rounding::same(50.0))
                .min_size(Vec2 {
                    x: ISLAND_SIZE,
                    y: ISLAND_SIZE,
                })
                .fill(Color32::WHITE),
            );
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
