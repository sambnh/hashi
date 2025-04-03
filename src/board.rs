use eframe::egui;

use crate::{
    bridge::Bridge,
    island::{Island, Position},
};

#[derive(Default)]
pub struct HashiBoard<'a> {
    pub islands: Vec<Island>,
    pub bridges: Vec<Bridge<'a>>,
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
