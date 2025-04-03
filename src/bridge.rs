use eframe::egui::{self, Color32, Pos2, Sense, Stroke, Vec2, Widget};

use crate::island::{Island, Position, ISLAND_SIZE};

const BRIDGE_OFFSET: f32 = ISLAND_SIZE / 4.0;

#[derive(Debug, Default, Clone, Copy)]
pub enum BridgeCount {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
pub struct Bridge<'a> {
    pub from: &'a Island,
    pub to: &'a Island,
    pub count: BridgeCount,
    direction: Direction,
}

#[derive(Debug, Clone)]
pub struct BridgeDirectionError;

impl Bridge<'_> {
    pub fn new<'a>(
        from: &'a Island,
        to: &'a Island,
        count: BridgeCount,
    ) -> Result<Bridge<'a>, BridgeDirectionError> {
        let direction = match <Position as Into<Pos2>>::into(to.position)
            - <Position as Into<Pos2>>::into(from.position)
        {
            Vec2 { x: 0.0, y: _ } => Direction::Vertical,
            Vec2 { x: _, y: 0.0 } => Direction::Horizontal,
            Vec2 { .. } => return Err(BridgeDirectionError),
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

        let points: [Pos2; 2] = [self.from.position.into(), self.to.position.into()];

        match (self.count, self.direction) {
            (BridgeCount::Zero, _) => (),
            (BridgeCount::One, _) => {
                painter.line_segment(points, Stroke::new(1.0, Color32::BLACK));
            }
            (BridgeCount::Two, Direction::Horizontal) => {
                painter.line_segment(
                    points.map(|mut point| {
                        point.y += BRIDGE_OFFSET;
                        point
                    }),
                    Stroke::new(1.0, Color32::BLACK),
                );
                painter.line_segment(
                    points.map(|mut point| {
                        point.y -= BRIDGE_OFFSET;
                        point
                    }),
                    Stroke::new(1.0, Color32::BLACK),
                );
            }
            (BridgeCount::Two, Direction::Vertical) => {
                painter.line_segment(
                    points.map(|mut point| {
                        point.x += BRIDGE_OFFSET;
                        point
                    }),
                    Stroke::new(1.0, Color32::BLACK),
                );
                painter.line_segment(
                    points.map(|mut point| {
                        point.x -= BRIDGE_OFFSET;
                        point
                    }),
                    Stroke::new(1.0, Color32::BLACK),
                );
            }
        }

        response
    }
}
