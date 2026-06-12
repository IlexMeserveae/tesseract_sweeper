use crate::minesweeper::coordinate::Coordinate;
use crate::minesweeper::Minefield;
use crate::tesseract::fonts::{reload_active_fonts, MONOSPACE_FONT, PROPORTIONAL_FONT, TITLE_FONT};
use crate::tesseract::AppPhase::*;
use eframe::egui::{Color32, Context, Key, Margin, RichText, ScrollArea};
use eframe::{egui, App, CreationContext, Frame};
use std::cmp::{min, PartialEq};
use std::ops::AddAssign;
use tile_settings::TileSettings;

mod icons;
mod colors;
mod fonts;
mod tile_settings;

#[derive(Default, Eq, PartialEq)]
enum AppPhase {
    #[default]
    MainMenu,
    SizeMenu,
    GameRunning,
    GameLost,
    GameWon,
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

    // Custom Play Menu
    pick_ord_sizes: [f32; 4],
    pick_mine_count: f32,
    
}

impl TesseractApp {
    pub fn new(cc: &CreationContext) -> Self {
        let ctx = &cc.egui_ctx;

        egui_extras::loaders::install_image_loaders(ctx);
        fonts::init_fonts();
        fonts::set_font(&PROPORTIONAL_FONT, "Hack Regular").unwrap();
        fonts::set_font(&MONOSPACE_FONT,"Hack Regular").unwrap();
        fonts::set_font(&TITLE_FONT, "Slice").unwrap();
        reload_active_fonts(ctx);

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

            pick_ord_sizes: [6.; 4],
            pick_mine_count: 30.,
        }
    }

    pub fn set_minefield(&mut self, minefield: Minefield) {
        self.minefield = minefield.into();
        self.next_phase = GameRunning.into();
    }
    pub fn clear_minefield(&mut self) {
        self.minefield = None;
        self.next_phase = MainMenu.into();
    }
    fn enable_dev_mode(&mut self) {
        self.dev_mode = true;
    }
    fn disable_dev_mode(&mut self) {
        self.dev_mode = false;
        self.inspected_tile = None;
        self.mouse_tool = None;
    }
}

mod top_bar {
    use crate::tesseract::TesseractApp;
    use eframe::egui;
    use eframe::egui::containers::menu;
    use eframe::egui::{Button, Context, RichText, Ui};

    impl TesseractApp {
        pub(super) fn show_top_bar(&mut self, ctx: &Context) {
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
}
mod dev_panel {
    use crate::tesseract::{MouseTool, TesseractApp};
    use eframe::egui;
    use eframe::egui::{Button, Context, RichText, Ui};

    impl TesseractApp {
        pub(super) fn show_dev_panel(&mut self, ctx: &Context) {
            egui::SidePanel::right("Side Panel").show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.show_inspect(ui);
                });
            });
        }

        fn show_inspect(&mut self, ui: &mut Ui) {
            ui.horizontal(|ui| {
                if ui.add(Button::new(RichText::new("Inspect Tile"))).clicked() {
                    self.mouse_tool.get_or_insert(MouseTool::DevTileInspect);
                };
                if self.mouse_tool == Some(MouseTool::DevTileInspect) {
                    if ui.add(Button::new(RichText::new("Cancel"))).clicked() {
                        self.mouse_tool = None;
                    }
                }
                else if self.inspected_tile.is_some() {
                    if ui.add(Button::new(RichText::new("Deselect"))).clicked() {
                        self.inspected_tile = None;
                    }
                }
            });

            if let Some(coord) = self.inspected_tile {
                let tile = self.minefield.as_ref().unwrap().index(coord);
                ui.vertical(|ui| {
                    ui.label(
                        format!("Tile at: {}", coord)
                    );
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
        }
    }
}
mod minefield {
    use crate::tesseract::minefield::Ordinate::X;
use super::icons::icon;
    use super::icons::Icon::{IncorrectFlag, Mine, RedFlag};
    use super::AppPhase::*;
    use super::{colors, hidden_background, minecount_text, revealed_background, MouseTool, TesseractApp};
    use crate::minesweeper::coordinate::{Coordinate, Ordinate};
    use crate::minesweeper::tile::TileError;
    use crate::minesweeper::{coordinate, QueryResult};
    use eframe::{egui, emath};
    use eframe::egui::{pos2, vec2, Button, Color32, Image, Margin, PointerButton, Rect, Response, RichText, ScrollArea, Sense, Stroke, Ui};
    use TileType::*;
    use crate::minesweeper::coordinate::Ordinate::Y;

    enum TileType {
        TextTile(RichText, Color32),
        ImageTile(Image<'static>, Color32),
    }

    impl TesseractApp {
        pub(super) const OUTER_MARGIN: i8 = 10;
        pub(super) const INNER_MARGIN: i8 = 10;
        pub fn display_scroller(&mut self, ui: &mut Ui) {
            let margin = Margin::symmetric(Self::OUTER_MARGIN, Self::OUTER_MARGIN);
            egui::Frame::new().inner_margin(margin).show(ui, |ui| {

                let scroll_area = ScrollArea::both().id_salt("Minefield Scroller");
                scroll_area.show_viewport(ui, |ui, rect| {

                    let margin = Margin::symmetric(Self::INNER_MARGIN, Self::INNER_MARGIN);
                    let visible = Rect::from_min_size(
                        pos2(0., 0.), rect.size() + vec2(20., 45.)
                        // - vec2(50., 50.)
                    );

                    egui::Frame::new().outer_margin(margin).show(ui, |ui| {
                        self.display_minefield(ui, visible);
                    });
                });
            });
        }
        pub fn display_minefield(&mut self, ui: &mut Ui, visible_rect: Rect) {
            let mf = self.minefield.as_ref().unwrap();
            let sub_height = self.settings.little_gap_size().y * (mf.length(Y) - 1) as f32 +
                mf.length(Y) as f32 * (self.settings.tile_size().y);
            let sub_width = self.settings.little_gap_size().x * (mf.length(X) - 1) as f32 +
                mf.length(X) as f32 * (self.settings.tile_size().x);

            let sub_size = vec2(sub_width, sub_height);
            let spacing = self.settings.big_gap_size();
            egui::Grid::new("minefield").spacing(spacing).show(ui, |ui| {
                for w in 1..=self.minefield.as_ref().unwrap().length(Ordinate::W) {
                    for z in 1..=self.minefield.as_ref().unwrap().length(Ordinate::Z) {

                        // Check if visible
                        let cursor = ui.cursor().min;
                        let sub_rect = Rect::from_min_size(cursor, sub_size);
                        if !visible_rect.intersects(sub_rect) {
                            // ui.painter().rect_filled(sub_rect, 2.0, Color32::GOLD);
                            ui.advance_cursor_after_rect(sub_rect);
                            continue;
                        }

                        let vis =
                            if visible_rect.contains_rect(sub_rect) { None }
                            else { Some(visible_rect) };
                        self.display_subfield(ui, w, z, vis);
                    }
                    ui.end_row();
                }
            });
        }
        fn display_subfield(&mut self, ui: &mut Ui, w: usize, z: usize, visible_rect: Option<Rect>) {
            let spacing = self.settings.little_gap_size();
            egui::Grid::new(format!("subfield-{z}-{w}")).spacing(spacing)
                .show(ui, |ui| {
                    for y in 1..=self.minefield.as_ref().unwrap().length(Ordinate::Y) {
                        for x in 1..=self.minefield.as_ref().unwrap().length(Ordinate::X) {
                            let coord = coordinate::coordinate(x, y, z, w);

                            if let Some(visible_rect) = visible_rect {
                                let cursor = ui.cursor().min;
                                let sub_rect = Rect::from_min_size(cursor, self.settings.tile_size());
                                if !visible_rect.intersects(sub_rect) {
                                    // ui.painter().rect_filled(sub_rect, 2.0, Color32::GOLD);
                                    ui.advance_cursor_after_rect(sub_rect);
                                    continue;
                                }
                            }

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
                QueryResult::Revealed(_) => {}
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
}

impl App for TesseractApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(next) = self.next_phase.take() {
            self.current_phase = next;
        }

        if self.current_phase == GameRunning && self.minefield.as_ref().unwrap().game_won() {
            self.current_phase = GameWon;
        }

        self.highlighted_tiles.clear();

        let _ = match self.current_phase {
            MainMenu => self.update_main_menu(ctx),
            SizeMenu => self.update_size_menu(ctx),
            GameRunning => {
                if let Some(coord) = self.hovered_tile.take() {
                    self.highlighted_tiles = self.minefield.as_ref().unwrap()
                        .get_neighbours(coord, 1);
                }

                self.update_game(ctx)
            }
            GameWon | GameLost => self.update_game(ctx),
        };
    }
}

mod update {
    use super::*;
    use crate::minesweeper::coordinate::{coordinate, ORDINATES};
    use crate::tesseract::fonts::{get_scale, title_family};
    use crate::{FieldSettings, Preset};
    use eframe::egui::Align::Center;
    use eframe::egui::{vec2, Align, AtomExt, Button, FontId, Key, Label, Layout, Response, Ui, ViewportCommand};
    use std::mem;
    use std::process::Command;
    use Align::Min;
    use crate::minesweeper::coordinate::Ordinate::*;

    pub(super) enum UpdateError {}
    type UpdateResult = Result<(), UpdateError>;


    impl TesseractApp {

        fn play_button(&mut self, ui: &mut Ui, text: &str, preset: Preset) {
            let play = Button::new(RichText::new(text).size(30.))
                .fill(Color32::from_gray(28));
            if ui.add(play).clicked() {
                let mut field = preset.generate();
                field.quickstart().unwrap();
                self.set_minefield(field);
            };
        }

        fn previous_menu_if_escaped(&mut self, ctx: &Context, prev_phase: AppPhase) -> bool {
            if any_pressed(ctx, vec![Key::Escape]) {
                self.next_phase = Some(prev_phase);
                true;
            }

            false
        }

        pub(super) fn update_main_menu(&mut self, ctx: &Context) -> UpdateResult {
            egui::CentralPanel::default().show(ctx, |ui| {
                let layout = Layout::top_down(Center);
                ui.with_layout(layout, |ui| {
                    let height = ui.available_height();

                    ui.add_space( height * 0.25);

                    let title = Label::new(RichText::new("Tesseract")
                        .font(FontId::new(100., title_family())));
                    ui.add(title);

                    ui.add_space(height * 0.10);

                    self.play_button(ui, " Play 2D ", Preset::Medium2D);
                    ui.add_space(height * 0.02);
                    self.play_button(ui, " Play Small ", Preset::Small4D);
                    ui.add_space(height * 0.02);
                    self.play_button(ui, " Play Medium ", Preset::Medium4D);
                    ui.add_space(height * 0.02);
                    self.play_button(ui, " Play Large ", Preset::Large4D);
                    ui.add_space(height * 0.02);

                    let play_custom = Button::new(RichText::new(" Play Custom ")
                        .size(30.)).fill(Color32::from_gray(28));
                    if ui.add(play_custom).clicked() {
                        self.next_phase = Some(SizeMenu)
                    }
                    ui.add_space(height * 0.02);

                    let exit = Button::new(RichText::new(" Quit ")
                        .size(30.)).fill(Color32::from_gray(28));
                    if ui.add(exit).clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                    ui.add_space(height * 0.02);
                });
            });

            Ok(())
        }

        pub(super) fn update_size_menu(&mut self, ctx: &Context) -> UpdateResult {
            egui::CentralPanel::default().show(ctx, |ui| {
                let layout = Layout::top_down(Center);
                ui.with_layout(layout, |ui| {
                    let height = ui.available_height();

                    ui.add_space( height * 0.25);
                    let temp = Label::new(RichText::new("Custom Game").size(50.));
                    ui.add(temp);

                    ui.add_space(height * 0.10);
                    let margin = 15.;
                    let spacing = 20.;
                    let input_width = 120.;
                    let input_height = 40.;
                    let size = vec2(spacing * 3. + input_width * 4. + margin * 2., 0.);
                    egui::Frame::new().inner_margin(margin).show(ui, |ui| {
                        let layout = Layout::left_to_right(Center);
                        ui.allocate_ui_with_layout(size, layout, |ui| {
                            ui.spacing_mut().item_spacing.x = spacing;
                            for i in 0..4 {
                                self.ordinate_box(ui, input_width, input_height, i)
                            }
                        })
                    });

                    Self::int_input(ui, "mines:", 200., input_height,
                                    1., 4096., 4, &mut self.pick_mine_count);

                    ui.add_space(height * 0.05);
                    let play_button = Button::new(RichText::new("Play").size(30.));
                    if ui.add(play_button).clicked() {
                        let ords = &self.pick_ord_sizes;
                        let coord = coordinate(ords[0] as usize, ords[1] as usize, ords[2] as usize, ords[3] as usize);
                        let mines = self.pick_mine_count as u16;
                        let settings = FieldSettings::new(coord, mines);
                        if settings.is_err() {
                            // TODO
                            println!("{}", settings.err().unwrap());
                            return;
                        }

                        let mut mf = Minefield::new(settings.unwrap());
                        mf.quickstart().unwrap();
                        self.set_minefield(mf);
                        return;
                    }
                })
            });

            self.previous_menu_if_escaped(ctx, MainMenu);

            Ok(())
        }

        ///
        ///
        /// Creates an integer input widget.
        ///
        ///
        fn int_input(ui: &mut Ui, label: &str, width: f32, height: f32, min: f32, max: f32,
                     padding: usize, value: &mut f32) {
            let mut decr = None;
            let mut incr = None;
            let mut inpt = None;
            let font_scale = get_scale(&PROPORTIONAL_FONT);

            let button_width = 18.;
            let value_width = 24. + 12. * padding as f32;

            let size = vec2(width, height);
            let layout = Layout::left_to_right(Center);
            let _ = ui.allocate_ui_with_layout(size, layout, |ui| {
                ui.set_width(width);
                ui.spacing_mut().item_spacing.x = 0.;

                let other = f32::min(button_width + value_width + button_width, width);
                ui.add_sized(vec2(width - other, height), Label::new(
                    RichText::new(label).size(24. * font_scale)
                ));

                // Decrement Button
                decr = ui.add_sized(vec2(button_width, button_width), Button::new(
                    RichText::new("<").size(20. * font_scale).atom_grow(true)
                )).into();

                // Value
                inpt = ui.add_sized(vec2(value_width, height), Label::new(RichText::new(
                    format!("{:^1$.0}", value, padding)).size(24. * font_scale))).into();

                // Increment Button
                incr = ui.add_sized(vec2(button_width, button_width), Button::new(
                    RichText::new(">").size(20. * font_scale).atom_grow(true)
                )).into();
            });

            if decr.unwrap().clicked() {
                let _ = mem::replace(value, f32::max(min, f32::round(*value - 1.)));
            }
            if incr.unwrap().clicked() {
                let _ = mem::replace(value, f32::min(max, f32::round(*value + 1.)));
            }

            const SCROLL_FACTOR: f32 = 0.02;
            if let Some(inpt) = inpt && inpt.hovered() {
                let scroll = inpt.ctx.input(|i| i.raw_scroll_delta);
                let delta = inpt.ctx.input(|i|
                    (scroll.x + scroll.y) * SCROLL_FACTOR
                        * if i.modifiers.ctrl { 10. } else { 1. }
                        * if i.modifiers.shift { 100. } else { 1. }
                );

                value.add_assign(delta);
                if *value < min { let _ = mem::replace(value, min); }
                if *value > max { let _ = mem::replace(value, max); }
            }

            // DEBUG
            // println!("Expected: {width}   Actual: {}", resp.response.rect.width())
        }

        fn ordinate_box(&mut self, ui: &mut Ui, width: f32, height: f32, index: usize) {
            let label = format!("{}:", ORDINATES[index].name());
            Self::int_input(ui, &label, width, height, 1., 64.,
                            2, &mut self.pick_ord_sizes[index]);
        }

        pub(super) fn update_game(&mut self, ctx: &Context) -> UpdateResult {
            self.show_top_bar(ctx);

            if self.dev_mode { self.show_dev_panel(ctx) }

            let mf = self.minefield.as_ref().unwrap();
            let width = (2 * Self::OUTER_MARGIN + 2 * Self::INNER_MARGIN) as f32 +
                self.settings.big_gap_size().x * (mf.length(Z) - 1) as f32 +
                mf.length(Z) as f32 * (
                    self.settings.little_gap_size().x * (mf.length(X) - 1) as f32 +
                    mf.length(X) as f32 * (
                        self.settings.tile_size().x
                    )
                );
            let height = (2 * Self::OUTER_MARGIN + 2 * Self::INNER_MARGIN) as f32 +
                self.settings.big_gap_size().y * (mf.length(W) - 1) as f32 +
                mf.length(W) as f32 * (
                    self.settings.little_gap_size().y * (mf.length(Y) - 1) as f32 +
                    mf.length(Y) as f32 * (
                        self.settings.tile_size().y
                    )
                );


            egui::CentralPanel::default().show(ctx, |ui| {
                let size = ui.available_size();
                let layout = Layout::left_to_right(Min);
                ui.allocate_ui_with_layout(size, layout, |ui| {
                    ui.spacing_mut().item_spacing = vec2(0., 0.);
                    let gap = f32::max(0., ui.available_width() - width);
                    ui.add_space(gap * 0.5);

                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing = vec2(0., 0.);
                        let gap = f32::max(0., ui.available_height() - height);
                        ui.add_space(gap * 0.5);

                        self.display_scroller(ui)
                    });
                })
            });

            if self.previous_menu_if_escaped(ctx, AppPhase::MainMenu) {
                // TODO: Continue Last Game
            }

            Ok(())
        }
    }
}



fn any_pressed(ctx: &Context, keys: Vec<Key>) -> bool {
    keys.iter().any(|&key| ctx.input(|i| i.key_pressed(key)))
}

///
///
/// Calls try_into() on input and panics if the conversion fails.
///
/// Useful when you want to call try_into() with a generic type parameter, and can't assert
/// that Error implements Display
///
/// # Examples
///
/// ```
/// fn decrement_num<T>(num: &mut T) where T: SubAssign + TryFrom<usize> {
///     num.sub_assign(cast::<usize, T>(1));
/// }
/// ```
// fn cast<A, B>(input: A) -> B where A: TryInto<B> {
//     input.try_into().unwrap_or_else(|_| panic!("Failed to convert input!"))
// }

fn revealed_background(minecount: i16) -> Color32 {
    if minecount < 0 { return Color32::from_rgb(200, 160, 200) }
    let t = min(minecount, 10) as f32 / 10.;
    colors::TILE_REVEALED.lerp_to_gamma(Color32::from_rgb(200, 20, 20), t)
}
fn hidden_background(minecount: i16) -> Color32 {
    if minecount < 0 { return Color32::from_rgb(125, 25, 80); }
    let t = min(minecount, 10) as f32 / 10.;
    colors::TILE_HIDDEN.lerp_to_gamma(Color32::from_rgb(100, 10, 10), t)
}
fn minecount_text(minecount: i16) -> RichText {
    let color = Color32::BLACK; // colors::minecount_color(minecount);
    RichText::new(minecount.to_string()).color(color).size(20.)
}