use crate::minesweeper::{coordinate, Minefield};
use crate::tesseract::TesseractApp;
use eframe::egui::ViewportBuilder;
use eframe::{run_native, NativeOptions};

pub(crate) mod tesseract;
pub(crate) mod minesweeper;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1050., 1050.])
            .with_maximized(true),
        ..Default::default()
    };

    run_native("Tesseract", options, Box::new(|cc|
        Ok(Box::new(TesseractApp::new(cc))))).unwrap();
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
            Presets::Small2D => coordinate::coordinate(8, 8, 1, 1),
            Presets::Medium2D => coordinate::coordinate(16, 16, 1, 1),
            Presets::Large2D => coordinate::coordinate(32, 32, 1, 1),
            Presets::Small3D => coordinate::coordinate(6, 6, 6, 1),
            Presets::Medium3D => coordinate::coordinate(10, 10, 10, 1),
            Presets::Large3D => coordinate::coordinate(16, 16, 16, 1),
            Presets::Small4D => coordinate::coordinate(4, 4, 4, 4),
            Presets::Medium4D => coordinate::coordinate(6, 6, 6, 6),
            Presets::Large4D => coordinate::coordinate(10, 10, 10, 10),
        };
        let mines = match self {
            Presets::Small2D => 10,
            Presets::Medium2D => 40,
            Presets::Large2D => 160,
            Presets::Small3D => 8,
            Presets::Medium3D => 32,
            Presets::Large3D => 128,
            Presets::Small4D => 10,
            Presets::Medium4D => 20,
            Presets::Large4D => 40,
        };

        Minefield::new(size, mines).unwrap()
    }
}
