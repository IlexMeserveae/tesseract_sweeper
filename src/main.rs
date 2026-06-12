use crate::minesweeper::{coordinate, Minefield};
use crate::tesseract::TesseractApp;
use eframe::egui::ViewportBuilder;
use eframe::{run_native, NativeOptions};
use crate::minesweeper::coordinate::Coordinate;

pub(crate) mod tesseract;
pub(crate) mod minesweeper;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1024., 1024.])
            .with_maximized(true),
        ..Default::default()
    };

    run_native("Tesseract", options, Box::new(|cc|
        Ok(Box::new(TesseractApp::new(cc))))).unwrap();
}

pub struct FieldSettings {
    size: Coordinate,
    mine_count: u16,
}

impl FieldSettings {
    pub fn new(size: Coordinate, mine_count: u16) -> Result<Self, String> {
        if 4 * mine_count as usize > size.multiply_out() {
            return Err("Too many mines for this size of grid.".to_string())
        }

        Ok(FieldSettings { size, mine_count })
    }
}

enum Preset {
    Small2D,
    Medium2D,
    Large2D,
    // Small3D,
    // Medium3D,
    // Large3D,
    Small4D,
    Medium4D,
    Large4D,
}
impl Preset {
    pub fn generate(&self) -> Minefield {
        let size = match self {
            Preset::Small2D => coordinate::coordinate(8, 8, 1, 1),
            Preset::Medium2D => coordinate::coordinate(16, 16, 1, 1),
            Preset::Large2D => coordinate::coordinate(32, 32, 1, 1),
            // Preset::Small3D => coordinate::coordinate(6, 6, 6, 1),
            // Preset::Medium3D => coordinate::coordinate(10, 10, 10, 1),
            // Preset::Large3D => coordinate::coordinate(16, 16, 16, 1),
            Preset::Small4D => coordinate::coordinate(4, 4, 4, 4),
            Preset::Medium4D => coordinate::coordinate(6, 6, 6, 6),
            Preset::Large4D => coordinate::coordinate(10, 10, 10, 10),
        };
        let mines = match self {
            Preset::Small2D => 10,
            Preset::Medium2D => 40,
            Preset::Large2D => 160,
            // Preset::Small3D => 8,
            // Preset::Medium3D => 32,
            // Preset::Large3D => 128,
            Preset::Small4D => 10,
            Preset::Medium4D => 40,
            Preset::Large4D => 320,
        };

        let settings = FieldSettings::new(size, mines).unwrap();
        Minefield::new(settings)
    }
}
