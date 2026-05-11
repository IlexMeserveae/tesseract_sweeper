use crate::minesweeper::coordinate::{Coordinate, Ordinate};
use crate::minesweeper::{coordinate, Minefield, QueryResult};
use crate::tesseract::AppPhase::*;
use crate::tesseract::TileType::{ImageTile, TextTile};
use eframe::egui::{containers::menu, Button, Color32, Context, Image, IntoAtoms, Margin, PointerButton, Response, RichText, ScrollArea, Stroke, Ui};
use eframe::{egui, App, Frame};
use icons::{icon, Icon::*};
use std::cmp::{min, PartialEq};
use tile_settings::TileSettings;
use crate::minesweeper::tile::TileError;

mod icons;
mod colors;
mod tile_settings;

#[derive(Default)]
enum AppPhase {
    #[default]
    NoGame,
    GameRunning,
    GameLost,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MouseTool {
    // Dev Mode Exclusive
    DevTileInspect
}

pub struct TesseractApp {
    minefield: Option<Minefield>,
    settings: TileSettings,

    current_phase: AppPhase,
    next_phase: Option<AppPhase>,

    hovered_tile: Option<Coordinate>,
    highlighted_tiles: Vec<Coordinate>,

    mouse_tool: Option<MouseTool>,

    dev_mode: bool,
    inspected_tile: Option<Coordinate>,
}
impl Default for TesseractApp {
    fn default() -> Self {
        Self {
            minefield: None,
            settings: TileSettings::default(),

            current_phase: AppPhase::default(),
            next_phase: None,

            hovered_tile: None,
            highlighted_tiles: vec![],

            mouse_tool: None,

            dev_mode: false,
            inspected_tile: None,
        }
    }
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
        // ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        //     ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let spacing = self.settings.big_gap_size();
                egui::Grid::new("minefield").spacing(spacing).show(ui, |ui| {
                    for w in 1..=self.minefield.as_ref().unwrap().length(Ordinate::W) {
                        for z in 1..=self.minefield.as_ref().unwrap().length(Ordinate::Z) {
                            self.display_subfield(ui, w, z);
                        }
                        ui.end_row();
                    }
                });
        //     });
        // });
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
        let inspected = self.inspected_tile == Some(coord);

        let query = match self.current_phase {
            GameRunning => field.query_tile(coord),
            GameLost => field.query_tile_gameover(coord),
            _ => unreachable!(),
        };
        let tile = match query {
            QueryResult::Blank => {
                TextTile(RichText::new(""), colors::TILE_REVEALED)
            }
            QueryResult::Revealed(minecount) => {
                TextTile(minecount_text(minecount), revealed_background(minecount))
            }
            QueryResult::Hidden => {
                TextTile(RichText::new(""), colors::TILE_HIDDEN)
            }
            QueryResult::Flagged => {
                ImageTile(icon(RedFlag), colors::TILE_HIDDEN)
            },
            QueryResult::Exploded => {
                ImageTile(icon(Mine), colors::TILE_EXPLODED)
            },

            // Debug
            QueryResult::DebugMarked => {
                TextTile(RichText::new("?"), Color32::GOLD)
            }

            // Game Over Exclusive
            QueryResult::GoHidden(minecount) => {
                TextTile(minecount_text(minecount), hidden_background(minecount))
            }
            QueryResult::GoCorrect => {
                ImageTile(icon(RedFlag), colors::TILE_GAME_OVER)
            }
            QueryResult::GoIncorrect => {
                ImageTile(icon(IncorrectFlag), Color32::BROWN) // colors::TILE_GAME_OVER)
            },
            QueryResult::GoMine => {
                ImageTile(icon(Mine), colors::TILE_GAME_OVER)
            },
        };

        let mut button = match tile {
            TextTile(text, mut color) => {
                if highlighted { color = color.gamma_multiply(0.8) }
                Button::new(text).fill(color)
            }
            ImageTile(image, mut color) => {
                if highlighted { color = color.gamma_multiply(0.8) }
                Button::new(image).fill(color)
            }
        };

        if inspected { button = button.stroke(Stroke::new(4., Color32::DARK_BLUE)); }
        let button= ui.add(button.min_size(size));

        // Behaviour
        if let Some(tool) = self.mouse_tool {
            match tool {
                MouseTool::DevTileInspect => {
                    if button.clicked() { self.inspected_tile = Some(coord); }
                },
            };
        }
        else { self.normal_tile_behavior(coord, &button); }

        if button.hovered() { self.hovered_tile = Some(coord); }
    }

    fn normal_tile_behavior(&mut self, coord: Coordinate, button: &Response) {
        let field = self.minefield.as_mut().unwrap();
        let query = match self.current_phase {
            GameRunning => field.query_tile(coord),
            GameLost => field.query_tile_gameover(coord),
            _ => unreachable!(),
        };

        match query {
            QueryResult::Blank => {}
            QueryResult::Revealed(minecount) => {}
            QueryResult::Hidden => {
                if button.clicked_by(PointerButton::Primary) {
                    match field.reveal(coord) {
                        Ok(_) => {},
                        Err(TileError::Exploded) => self.next_phase = Some(GameLost),
                        Err(err) => println!("{:?} : {:?}", coord, err),
                    }
                }
                if button.clicked_by(PointerButton::Secondary) {
                    field.toggle_flagged(coord);
                }
            }
            QueryResult::Flagged => {
                if button.clicked_by(PointerButton::Secondary) {
                    field.toggle_flagged(coord);
                }
            }
            QueryResult::Exploded => {}

            // Debug
            QueryResult::DebugMarked => {}

            // GoExclusive
            _ => {}
        }
    }
}

enum TileType {
    TextTile(RichText, Color32),
    ImageTile(Image<'static>, Color32),
}

impl TesseractApp {
    fn show_dev_bar(&mut self, ctx: &Context) {
        egui::SidePanel::right("Side Panel").show(ctx, |ui| {
            ui.vertical(|ui| {

                ui.horizontal(|ui| {
                    if ui.add(Button::new(RichText::new("Inspect Tile"))).clicked() {
                        self.mouse_tool.get_or_insert(MouseTool::DevTileInspect);
                    };
                    if self.mouse_tool == Some(MouseTool::DevTileInspect) {
                        if ui.add(Button::new(RichText::new("Cancel"))).clicked() {
                            self.mouse_tool = None;
                        }
                    }
                });

                if let Some(coord) = self.inspected_tile {
                    let tile = self.minefield.as_ref().unwrap().index(coord);
                    ui.vertical(|ui| {
                        ui.label(
                            format!("True minecount: {}", tile.true_minecount())
                        );
                        ui.label(
                            format!("Hidden neighbours: {}", tile.hidden_neighbours())
                        );
                        ui.label(
                            format!("Has mine: {}", tile.has_mine())
                        );
                    });
                }
            });
        });
    }
}

impl TesseractApp {
    fn show_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel")
            .show(ctx, |ui| {
                egui::containers::menu::MenuBar::new().ui(ui, |ui| {
                    self.show_top_bar_buttons(ui)
                })
            });
    }
    fn show_top_bar_buttons(&mut self, ui: &mut Ui) {
        self.show_dev_mode_button(ui);

        ui.add(egui::Separator::default().spacing(20.));

        let msg = if self.dev_mode { "Dev Mode Enabled" }
        else { "Dev Mode Disabled" };
        ui.label(msg);
    }
    fn show_dev_mode_button(&mut self, ui: &mut Ui) {
        menu::MenuButton::new(RichText::new("Dev Mode").size(16.))
            .ui(ui, |ui| {
                if self.dev_mode {
                    let button = Button::new(RichText::new("Disable").size(14.));
                    if ui.add(button).clicked() {
                        self.dev_mode = false;
                        ui.close_menu();
                    }
                }
                else {
                    let button = Button::new(RichText::new("Enable").size(14.));
                    if ui.add(button).clicked() {
                        self.dev_mode = true;
                        ui.close_menu();
                    }
                }
            });
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

        self.show_top_bar(ctx);

        if self.dev_mode { self.show_dev_bar(ctx) }

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.|ui| {
                egui::Frame::new().inner_margin(Margin::symmetric(16, 16))
                    // .fill(Color32::GREEN)
                    .show(ui, |ui| {
                    ScrollArea::both().show(ui, |ui| {
                        match self.current_phase {
                            GameRunning | GameLost => self.display_minefield(ui),
                            _ => {}
                        }
                    });
                });
            // });
        });
    }
}

fn color_lerp(c1: Color32, c2: Color32, t: f32) -> Color32 {
    if t < 0. { return c1 } else if t > 1. { return c2 }
    Color32::from_rgb(
        (t * c2.r() as f32 + (1. - t) * c1.r() as f32) as u8,
        (t * c2.g() as f32 + (1. - t) * c1.g() as f32) as u8,
        (t * c2.b() as f32 + (1. - t) * c1.b() as f32) as u8,
    )
}

fn revealed_background(minecount: i16) -> Color32 {
    if minecount < 0 { return Color32::from_rgb(200, 160, 200) }
    let t = min(minecount, 10) as f32 / 10.;
    color_lerp(colors::TILE_REVEALED, Color32::from_rgb(200, 20, 20), t)
}
fn hidden_background(minecount: i16) -> Color32 {
    if minecount < 0 { return Color32::from_rgb(125, 25, 80); }
    let t = min(minecount, 10) as f32 / 10.;
    color_lerp(colors::TILE_HIDDEN, Color32::from_rgb(100, 10, 10), t)
}

fn minecount_text(minecount: i16) -> RichText {
    let color = Color32::BLACK; // colors::minecount_color(minecount);
    RichText::new(minecount.to_string()).color(color).size(20.)
}