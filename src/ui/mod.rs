use crate::game::{coordinate, Coordinate, Minefield, Ordinate, QueryResult};
use eframe::egui::{vec2, Align, AtomExt, Button, Color32, Context, Layout, Margin, PointerButton, RichText, ScrollArea, Ui, Vec2};
use eframe::{egui, App, Frame};

#[derive(Default)]
pub struct MinesweeperApp {
    minefield: Option<Minefield>,
    settings: UiSettings,
    lost_game: bool,
}

impl MinesweeperApp {
    pub fn set_minefield(&mut self, minefield: Option<Minefield>) {
        self.minefield = minefield;
    }
}

impl App for MinesweeperApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout::default()
                .with_main_align(Align::Center).with_cross_align(Align::Center);
            ui.with_layout(layout, |ui| {
                ScrollArea::both().show(ui, |ui| {
                    if let Some(minefield) = self.minefield.as_mut() {
                        let result = display_minefield(ui, &self.settings, minefield);
                        if result.lost_game { self.lost_game = true }
                    }
                    else {
                        // Blank
                    }
                });
            });
        });
    }
}

struct MinefieldResponse {
    lost_game: bool,
}

fn display_minefield(ui: &mut Ui, settings: &UiSettings, field: &mut Minefield) -> MinefieldResponse {
    let mut response = MinefieldResponse { lost_game: false };


    let layout = Layout::default()
        .with_main_align(Align::Center).with_cross_align(Align::Center);
    ui.with_layout(layout, |ui| {
        egui::Frame::new().inner_margin(Margin::symmetric(12, 12))
            .show(ui, |ui| {
            let spacing = settings.big_gap_size();
            egui::Grid::new("minefield").spacing(spacing).show(ui, |ui| {
                for w in 1..=field.length(Ordinate::W) {
                    for z in 1..=field.length(Ordinate::Z) {
                        let result = display_subfield(ui, &settings, field, w, z);
                        if result.lost_game { response.lost_game = true; }
                    }
                    ui.end_row();
                }
            });
        });
    });

    response
}

struct SubfieldResponse {
    lost_game: bool,
}

fn display_subfield(ui: &mut Ui, settings: &UiSettings, field: &mut Minefield, w: usize, z: usize) -> SubfieldResponse {
    let mut response = SubfieldResponse { lost_game: false };

    let spacing = settings.little_gap_size();
    egui::Grid::new(format!("subfield-{z}-{w}")).spacing(spacing)
        .show(ui, |ui| {
        for y in 1..=field.length(Ordinate::Y) {
            for x in 1..=field.length(Ordinate::X) {
                let coord = coordinate(x, y, z, w);
                let result = display_tile(ui, &settings, field, coord);
                if result.lost_game { response.lost_game = true };
            }
            ui.end_row();
        }
    });

    response
}

struct TileResponse {
    lost_game: bool,
}
fn display_tile(ui: &mut Ui, settings: &UiSettings, field: &mut Minefield,
                coord: Coordinate) -> TileResponse {
    let mut response = TileResponse { lost_game: false };

    let size = settings.tile_size();
    let query = field.query_tile(coord);
    match query {
        QueryResult::Revealed(0) => {
            let bg_color = Color32::GRAY;
            let _button = ui.add(Button::new(RichText::new(""))
                .fill(bg_color).min_size(size));
        }
        QueryResult::Revealed(minecount) => {
            let color = match minecount {
                x if x <= -1 => Color32::PURPLE,
                x if x ==  1 => Color32::DARK_GREEN,
                x if x <=  2 => Color32::GREEN,
                x if x <=  4 => Color32::YELLOW,
                x if x <=  6 => Color32::ORANGE,
                x if x <= 10 => Color32::RED,
                _ => Color32::DARK_RED,
            };
            let bg_color = Color32::GRAY;

            let _button = ui.add(Button::new(RichText::new(minecount.to_string())
                .color(color)).fill(bg_color).min_size(size));
        }
        QueryResult::Blank => {
            let bg_color = Color32::DARK_GRAY;
            let button = ui.add(Button::new(RichText::new(""))
                .fill(bg_color).min_size(size));

            if button.clicked_by(PointerButton::Primary) {
                if !field.reveal(coord).is_err() { response.lost_game = true; }
            }
            if button.clicked_by(PointerButton::Secondary) {
                field.toggle_flagged(coord);
            }
        },
        QueryResult::Flagged => {
            let bg_color = Color32::DARK_RED;
            let button = ui.add(Button::new(RichText::new(""))
                .fill(bg_color).min_size(size));
            
            if button.clicked_by(PointerButton::Secondary) {
                field.toggle_flagged(coord);
            }
        },
        QueryResult::Mine => {
            let bg_color = Color32::BLACK;
            let button = ui.add(Button::new(RichText::new(""))
                .fill(bg_color).min_size(size));
        },
    };

    response
}


pub struct UiSettings {
    hor_tile_scaling: f32,
    ver_tile_scaling: f32,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self { hor_tile_scaling: 0.50, ver_tile_scaling: 0.50 }
    }
}

impl UiSettings {
    const TILE: u16 = 90;
    pub fn tile_size(&self) -> Vec2 {
        vec2(Self::TILE as f32 * self.hor_tile_scaling,
             Self::TILE as f32 * self.ver_tile_scaling)
    }

    const LITTLE_GAP: u16 = 10;
    pub fn little_gap_size(&self) -> Vec2 {
        vec2(Self::LITTLE_GAP as f32 * self.hor_tile_scaling,
             Self::LITTLE_GAP as f32 * self.ver_tile_scaling)
    }

    const BIG_GAP: u16 = 100;
    pub fn big_gap_size(&self) -> Vec2 {
        vec2(Self::BIG_GAP as f32 * self.hor_tile_scaling,
             Self::BIG_GAP as f32 * self.ver_tile_scaling)
    }
}