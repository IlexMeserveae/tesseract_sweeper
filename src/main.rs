use eframe::egui::ViewportBuilder;
use eframe::{run_native, NativeOptions};
use crate::game::{coordinate, Minefield};
use crate::ui::MinesweeperApp;

fn main() {
    let mf = Minefield::new(coordinate(4, 4, 4, 4), 8);
    let mut app = MinesweeperApp::default();
    app.set_minefield(mf.ok());
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1050., 1050.]),
        ..Default::default()
    };

    run_native("4D Minesweeper", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}

pub(crate) mod ui;
pub(crate) mod game;