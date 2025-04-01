use std::{
    num,
    ops::{Add, Sub},
};

use eframe::{
    egui::{self, Color32, Label, Pos2, Sense, Shape, Stroke, Vec2, Widget},
    epaint::TextShape,
};

const ISLAND_SIZE: f32 = 8.0;
const ISLAND_SPACING: f32 = 50.0;
const BRIDGE_OFFSET: f32 = ISLAND_SIZE / 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
    x: usize,
    y: usize,
}

impl Into<Pos2> for Position {
    fn into(self) -> Pos2 {
        Pos2::new(
            self.x as f32 * ISLAND_SPACING + ISLAND_SPACING,
            self.y as f32 * ISLAND_SPACING + ISLAND_SPACING,
        )
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.abs_diff(rhs.x),
            y: self.y.abs_diff(rhs.y),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Island {
    position: Position,
    required_bridges: usize,
    connected_bridges: usize,
}

impl Widget for Island {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let label = Label::new(self.required_bridges.to_string()).sense(Sense::click());
        let (_, galley, _) = label.layout_in_ui(ui);
        let (_, response) = ui.allocate_at_least(
            Vec2 {
                x: ISLAND_SIZE * 2.0,
                y: ISLAND_SIZE * 2.0,
            },
            Sense::click(),
        );

        let painter = ui.painter();
        let position = self.position.into();

        let text_pos = position - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);

        painter.extend(vec![
            Shape::circle_filled(position, ISLAND_SIZE, Color32::WHITE),
            Shape::circle_stroke(position, ISLAND_SIZE, Stroke::new(1.0, Color32::BLACK)),
            Shape::Text(TextShape::new(text_pos, galley, Color32::BLACK)),
        ]);

        response
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum BridgeCount {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
struct Bridge<'a> {
    from: &'a Island,
    to: &'a Island,
    count: BridgeCount,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct BridgeDirectionError;

impl Bridge<'_> {
    fn new<'a>(
        from: &'a Island,
        to: &'a Island,
        count: BridgeCount,
    ) -> Result<Bridge<'a>, BridgeDirectionError> {
        let direction = match to.position - from.position {
            Position { x: 0, y: _ } => Direction::Vertical,
            Position { x: _, y: 0 } => Direction::Horizontal,
            Position { .. } => return Err(BridgeDirectionError),
        };

        Ok(Bridge {
            from,
            to,
            count,
            direction,
        })
    }
}

impl Widget for Bridge<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (_, response) = ui.allocate_at_least(
            Vec2 {
                x: ISLAND_SIZE * 2.0,
                y: ISLAND_SIZE * 2.0,
            },
            Sense::click(),
        );

        let painter = ui.painter();

        match self.count {
            BridgeCount::Zero => (),
            BridgeCount::One => {
                painter.line_segment(
                    [self.from.position.into(), self.to.position.into()],
                    Stroke::new(1.0, Color32::BLACK),
                );
            }
            BridgeCount::Two => {
                painter.line_segment(
                    [self.from.position.into(), self.to.position.into()],
                    Stroke::new(1.0, Color32::BLACK),
                );
                painter.line_segment(
                    [self.from.position.into(), self.to.position.into()],
                    Stroke::new(1.0, Color32::BLACK),
                );
            }
        }

        response
    }
}

#[derive(Default)]
struct HashiBoard<'a> {
    islands: Vec<Island>,
    bridges: Vec<Bridge<'a>>,
}

impl HashiBoard<'_> {
    fn add_island(&mut self, position: Position, required_bridges: usize) {
        todo!();
    }

    fn add_bridge(&mut self, pos1: Position, pos2: Position) {
        todo!();
    }
}

impl eframe::App for HashiBoard<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for bridge in &self.bridges {
                ui.add(*bridge);
            }
            for island in &self.islands {
                ui.add(*island);
            }
        });
    }
}

fn main() -> eframe::Result {
    let islands = [
        Island {
            position: Position { x: 0, y: 0 },
            required_bridges: 1,
            connected_bridges: 1,
        },
        Island {
            position: Position { x: 0, y: 1 },
            required_bridges: 4,
            connected_bridges: 4,
        },
        Island {
            position: Position { x: 1, y: 1 },
            required_bridges: 1,
            connected_bridges: 1,
        },
        Island {
            position: Position { x: 0, y: 2 },
            required_bridges: 1,
            connected_bridges: 1,
        },
    ];
    let bridges = [
        Bridge::new(&islands[0], &islands[1], BridgeCount::One),
        Bridge::new(&islands[1], &islands[2], BridgeCount::One),
        Bridge::new(&islands[1], &islands[3], BridgeCount::Two),
    ]
    .map(|result| result.unwrap());
    let hashi_board = HashiBoard {
        islands: Vec::from(islands),
        bridges: Vec::from(bridges),
    };

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Hashi", options, Box::new(|_| Ok(Box::from(hashi_board))))
}
