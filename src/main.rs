use board::HashiBoard;
use bridge::{Bridge, BridgeCount};
use eframe::egui::{self};
use island::{Island, Position};

mod board;
mod bridge;
mod island;

fn main() -> eframe::Result {
    let islands = [
        Island {
            position: Position { x: 0, y: 0 },
            required_bridges: 1,
        },
        Island {
            position: Position { x: 0, y: 1 },
            required_bridges: 4,
        },
        Island {
            position: Position { x: 1, y: 1 },
            required_bridges: 1,
        },
        Island {
            position: Position { x: 0, y: 2 },
            required_bridges: 4,
        },
        Island {
            position: Position { x: 1, y: 2 },
            required_bridges: 2,
        },
    ];
    // TODO: Come up with a better way to handle these mutable references.
    let mut bridge_count0 = BridgeCount::One;
    let mut bridge_count1 = BridgeCount::One;
    let mut bridge_count2 = BridgeCount::Two;
    let mut bridge_count3 = BridgeCount::Two;
    let bridges = [
        Bridge::new(&islands[0], &islands[1], &mut bridge_count0),
        Bridge::new(&islands[1], &islands[2], &mut bridge_count1),
        Bridge::new(&islands[1], &islands[3], &mut bridge_count2),
        Bridge::new(&islands[3], &islands[4], &mut bridge_count3),
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
