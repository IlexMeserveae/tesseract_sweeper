use crate::app::MinesweeperApp;
use crate::game::{coordinate, Minefield};
use crate::Presets::Medium3D;
use eframe::egui::ViewportBuilder;
use eframe::{run_native, NativeOptions};

fn main() {
    let mut mf = Medium3D.generate();
    mf.quickstart().unwrap();
    let mut app = MinesweeperApp::default();
    app.set_minefield(mf.into());
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1050., 1050.]),
        ..Default::default()
    };

    run_native("4D Minesweeper", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}

enum Presets {
    Small2D,
    Medium2D,
    Large2D,
    Small3D,
    Medium3D,
    Large3D,
    Small4D,
    Medium4D,
    Large4D,
}
impl Presets {
    pub fn generate(&self) -> Minefield {
        let size = match self {
            Presets::Small2D => coordinate(8, 8, 1, 1),
            Presets::Medium2D => coordinate(16, 16, 1, 1),
            Presets::Large2D => coordinate(32, 32, 1, 1),
            Presets::Small3D => coordinate(6, 6, 6, 1),
            Presets::Medium3D => coordinate(10, 10, 10, 1),
            Presets::Large3D => coordinate(16, 16, 16, 1),
            Presets::Small4D => coordinate(4, 4, 4, 4),
            Presets::Medium4D => coordinate(6, 6, 6, 6),
            Presets::Large4D => coordinate(10, 10, 10, 10),
        };
        let mines = match self {
            Presets::Small2D => 10,
            Presets::Medium2D => 40,
            Presets::Large2D => 160,
            Presets::Small3D => 8,
            Presets::Medium3D => 32,
            Presets::Large3D => 128,
            Presets::Small4D => 8,
            Presets::Medium4D => 16,
            Presets::Large4D => 32,
        };

        Minefield::new(size, mines).unwrap()
    }
}

pub(crate) mod app;
pub(crate) mod game;