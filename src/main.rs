use eframe::{egui::{
    self, Color32, Label, Pos2, Sense, Shape, Stroke, Vec2, Widget 
}, epaint::TextShape};

const ISLAND_SIZE: f32 = 8.0;
const ISLAND_SPACING: f32 = 50.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
    x: usize,
    y: usize,
}

impl Into<Pos2> for Position {
    fn into(self) -> Pos2 {
        Pos2::new(self.x as f32 * ISLAND_SPACING + ISLAND_SPACING, self.y as f32 * ISLAND_SPACING + ISLAND_SPACING)
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
        let (_, response) = ui.allocate_at_least(Vec2 { x: ISLAND_SIZE*2.0, y: ISLAND_SIZE*2.0}, Sense::click());
        
        let painter = ui.painter();
        let position = self.position.into();

        let text_pos = position - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);
            
        painter.extend(vec![
            Shape::circle_stroke(position, ISLAND_SIZE, Stroke::new(1.0, Color32::BLACK)),
            Shape::Text(TextShape::new(text_pos, galley, Color32::BLACK))
        ]);

        response
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum BridgeCount {
    #[default]
    ZERO = 0,
    ONE = 1,
    TWO = 2,
}

#[derive(Debug, Clone, Copy)]

struct Bridge<'a> {
    from: &'a Island,
    to: &'a Island,
    count: BridgeCount
}


impl Widget for Bridge<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (_, response) = ui.allocate_at_least(Vec2 { x: ISLAND_SIZE*2.0, y: ISLAND_SIZE*2.0}, Sense::click());
        
        let painter = ui.painter();

        painter.line_segment( [self.from.position.into(), self.to.position.into()], Stroke::new(1.0, Color32::BLACK));

        response
    }
}

#[derive(Default)]
struct HashiBoard<'a> {
    islands: Vec<Island>,
    bridges: Vec<Bridge<'a>>
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
                connected_bridges: 0,
            },
            Island {
                position: Position { x: 0, y: 1 },
                required_bridges: 2,
                connected_bridges: 0,
            },
            Island {
                position:Position { x: 1, y: 1 },
                required_bridges: 1,
                connected_bridges: 0,
            },
        ];
    let bridges= [
        Bridge {
            from: &islands[0], to: &islands[1], count: BridgeCount::ONE
        },
        Bridge {
            from: &islands[1], to: &islands[2], count: BridgeCount::ONE
        },
    ];
    let hashi_board = HashiBoard {
        islands: Vec::from(islands),
        bridges: Vec::from(bridges)
    };

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hashi",
        options,
        Box::new(|_| Ok(Box::from(hashi_board))),
    )
}
