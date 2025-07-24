use eframe::egui::{self, Pos2, Rect, Sense, Vec2, Widget};

use crate::island::{Island, Position, ISLAND_SIZE};

const BRIDGE_OFFSET: f32 = ISLAND_SIZE / 4.0;

#[derive(Debug, Default, Clone, Copy, Hash)]
pub enum BridgeCount {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
}

impl BridgeCount {
    fn next(self) -> Self {
        match self {
            BridgeCount::Zero => BridgeCount::One,
            BridgeCount::One => BridgeCount::Two,
            BridgeCount::Two => BridgeCount::Zero,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Hash)]
pub struct Bridge<'a> {
    pub from: &'a Island,
    pub to: &'a Island,
    pub count: &'a mut BridgeCount,
    direction: Direction,
}

#[derive(Debug, Clone)]
pub struct BridgeDirectionError;

impl Bridge<'_> {
    pub fn new<'a>(
        from: &'a Island,
        to: &'a Island,
        count: &'a mut BridgeCount,
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

impl Widget for &mut Bridge<'_> {
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

        let point_rect = Rect::from_two_pos(points[0], points[1]).expand(ISLAND_SIZE);
        let point_id = response.id.with(&self);
        let point_response = ui.interact(point_rect, point_id, Sense::drag());

        let stroke = ui.style().interact(&point_response).fg_stroke;

        if point_response.drag_stopped() {
            *self.count = self.count.next();
        }

        match (&self.count, self.direction) {
            (BridgeCount::Zero, _) => (),
            (BridgeCount::One, _) => {
                painter.line_segment(points, stroke);
            }
            (BridgeCount::Two, Direction::Horizontal) => {
                painter.line_segment(
                    points.map(|mut point| {
                        point.y += BRIDGE_OFFSET;
                        point
                    }),
                    stroke,
                );
                painter.line_segment(
                    points.map(|mut point| {
                        point.y -= BRIDGE_OFFSET;
                        point
                    }),
                    stroke,
                );
            }
            (BridgeCount::Two, Direction::Vertical) => {
                painter.line_segment(
                    points.map(|mut point| {
                        point.x += BRIDGE_OFFSET;
                        point
                    }),
                    stroke,
                );
                painter.line_segment(
                    points.map(|mut point| {
                        point.x -= BRIDGE_OFFSET;
                        point
                    }),
                    stroke,
                );
            }
        }

        response
    }
}
