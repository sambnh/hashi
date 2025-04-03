use std::ops::Add;

use eframe::{
    egui::{self, Color32, Label, Pos2, Sense, Shape, Stroke, Vec2, Widget},
    epaint::TextShape,
};

pub const ISLAND_SIZE: f32 = 8.0;
pub const ISLAND_SPACING: f32 = 50.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<Position> for Pos2 {
    fn from(position: Position) -> Self {
        Pos2::new(
            position.x as f32 * ISLAND_SPACING + ISLAND_SPACING,
            position.y as f32 * ISLAND_SPACING + ISLAND_SPACING,
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

#[derive(Debug, Default, Clone, Copy)]
pub struct Island {
    pub position: Position,
    pub required_bridges: usize,
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
