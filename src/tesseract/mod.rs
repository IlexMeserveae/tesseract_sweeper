use crate::minesweeper::{coordinate, Minefield, QueryResult};
use eframe::egui::{vec2, Align, Button, Color32, Context, Image, Layout, Margin, PointerButton, Response, RichText, ScrollArea, Ui, Vec2};
use eframe::{egui, App, Frame};
use icons::{icon, Icon::*};
use crate::app::AppPhase::*;
use crate::minesweeper::coordinate::{Coordinate, Ordinate};

mod icons;
mod colors;

#[derive(Default)]
enum AppPhase {
    #[default]
    NoGame,
    GameRunning,
    GameLost,
}

#[derive(Default)]
pub struct TesseractApp {
    minefield: Option<Minefield>,
    settings: TileSettings,

    current_phase: AppPhase,
    next_phase: Option<AppPhase>,

    hovered_tile: Option<Coordinate>,
    highlighted_tiles: Vec<Coordinate>
}
impl TesseractApp {
    pub fn set_minefield(&mut self, minefield: Minefield) {
        self.minefield = minefield.into();
        self.next_phase = GameRunning.into();
    }
    pub fn clear_minefield(&mut self) {
        self.minefield = None;
        self.next_phase = NoGame.into();
    }

    fn display_minefield(&mut self, ui: &mut Ui) {
        let layout = Layout::default()
            .with_main_align(Align::Center).with_cross_align(Align::Center);
        ui.with_layout(layout, |ui| {
            egui::Frame::new().inner_margin(Margin::symmetric(12, 12))
                .show(ui, |ui| {
                    let spacing = self.settings.big_gap_size();
                    egui::Grid::new("minefield").spacing(spacing).show(ui, |ui| {
                        for w in 1..=self.minefield.as_ref().unwrap().length(Ordinate::W) {
                            for z in 1..=self.minefield.as_ref().unwrap().length(Ordinate::Z) {
                                self.display_subfield(ui, w, z);
                            }
                            ui.end_row();
                        }
                    });
                });
        });
    }
    fn display_subfield(&mut self, ui: &mut Ui, w: usize, z: usize) {
        let spacing = self.settings.little_gap_size();
        egui::Grid::new(format!("subfield-{z}-{w}")).spacing(spacing)
            .show(ui, |ui| {
                for y in 1..=self.minefield.as_ref().unwrap().length(Ordinate::Y) {
                    for x in 1..=self.minefield.as_ref().unwrap().length(Ordinate::X) {
                        let coord = coordinate::coordinate(x, y, z, w);
                        self.display_tile(ui, coord);
                    }
                    ui.end_row();
                }
            });
    }
    fn display_tile(&mut self, ui: &mut Ui, coord: Coordinate) {
        let field = self.minefield.as_mut().unwrap();
        let size = self.settings.tile_size();
        let highlighted = self.highlighted_tiles.contains(&coord);
        let query = match self.current_phase {
            GameRunning => field.query_tile(coord),
            GameLost => field.query_tile_gameover(coord),
            _ => unreachable!(),
        };
        let button = match query {
            QueryResult::Revealed(minecount) => {
                text_tile(ui, size, minecount_text(minecount), colors::TILE_REVEALED, highlighted)
            }
            QueryResult::Blank => {
                let button = text_tile(ui, size, RichText::new(""),
                                       colors::TILE_BLANK, highlighted);

                if button.clicked_by(PointerButton::Primary) {
                    if field.reveal(coord).is_err() { self.next_phase = GameLost.into(); }
                }
                if button.clicked_by(PointerButton::Secondary) {
                    field.toggle_flagged(coord);
                }
                button
            },
            QueryResult::Flagged => {
                let button = image_tile(ui, size, icon(RedFlag), colors::TILE_BLANK,
                                        highlighted);

                if button.clicked_by(PointerButton::Secondary) {
                    field.toggle_flagged(coord);
                }
                button
            },
            QueryResult::Exploded => {
                image_tile(ui, size, icon(Mine), colors::TILE_EXPLODED, highlighted)
            },
            // Game Over Exclusive
            QueryResult::GoUnrevealed(minecount) => {
                text_tile(ui, size, minecount_text(minecount), colors::TILE_GAME_OVER, highlighted)
            }
            QueryResult::GoCorrect => {
                image_tile(ui, size, icon(RedFlag), colors::TILE_GAME_OVER, highlighted)
            }
            QueryResult::GoIncorrect => {
                image_tile(ui, size, icon(IncorrectFlag), colors::TILE_GAME_OVER, highlighted)
            },
            QueryResult::GoMine => {
                image_tile(ui, size, icon(Mine), colors::TILE_GAME_OVER, highlighted)
            },
        };

        if button.hovered() { self.hovered_tile = Some(coord); }
    }
}
impl App for TesseractApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(next) = self.next_phase.take() {
            self.current_phase = next;
        }

        self.highlighted_tiles.clear();
        if let Some(coord) = self.hovered_tile.take() {
            match self.current_phase {
                GameRunning => {
                    self.highlighted_tiles = self.minefield.as_ref().unwrap()
                        .get_neighbours(coord, 1);
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = Layout::top_down(Align::Center).with_main_align(Align::Center);
            ui.with_layout(layout, |ui| {
                ScrollArea::both().show(ui, |ui| {
                    match self.current_phase {
                        GameRunning | GameLost => self.display_minefield(ui),
                        _ => {}
                    }
                });
            });
        });
    }
}

fn text_tile<'a>(ui: &mut Ui, size: Vec2, text: RichText, mut color: Color32, highlight: bool)
    -> Response {
    if highlight { color = color.gamma_multiply(0.8) }
    ui.add(Button::new(text).fill(color).min_size(size))
}
fn image_tile<'a>(ui: &mut Ui, size: Vec2, image: Image, mut color: Color32, highlight: bool)
    -> Response {
    if highlight { color = color.gamma_multiply(0.8) }
    ui.add(Button::new(image).fill(color).min_size(size))
}

fn minecount_text(minecount: i16) -> RichText {
    if minecount == 0 { return RichText::new(""); }
    let color = match minecount {
        x if x <= -1 => colors::MINECOUNT_NEGATIVE,
        x if x ==  1 => colors::MINECOUNT_ONE,
        x if x <=  2 => colors::MINECOUNT_TWO,
        x if x <=  4 => colors::MINECOUNT_FOUR,
        x if x <=  6 => colors::MINECOUNT_SIX,
        x if x <= 10 => colors::MINECOUNT_TEN,
        _ => colors::MINECOUNT_MAX,
    };
    RichText::new(minecount.to_string()).color(color)
}

pub struct TileSettings {
    hor_tile_scaling: f32,
    ver_tile_scaling: f32,
}
impl Default for TileSettings {
    fn default() -> Self {
        Self { hor_tile_scaling: 0.50, ver_tile_scaling: 0.50 }
    }
}
impl TileSettings {
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