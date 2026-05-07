use eframe::egui::Color32;
use std::mem;
use std::sync::{LazyLock, Mutex};

pub static TILE_HIDDEN: Color32 = Color32::DARK_GRAY;
pub static TILE_REVEALED: Color32 = Color32::LIGHT_GRAY;
pub static TILE_EXPLODED: Color32 = Color32::RED;
pub static TILE_GAME_OVER: Color32 = Color32::GRAY;

static MC_COLOR_MAP: LazyLock<Mutex<MinecountColorMap>> = LazyLock::new(|| Mutex::default());

pub fn minecount_color(minecount: i16) -> Color32 {
    MC_COLOR_MAP.lock().unwrap().get(minecount)
}
pub fn set_minecount_color_map(map: MinecountColorMap) {
    _ = mem::replace(&mut *MC_COLOR_MAP.lock().unwrap(), map);
}
pub fn get_minecount_color_map() -> MinecountColorMap {
    MC_COLOR_MAP.lock().unwrap().clone()
}

#[derive(Clone)]
pub struct MinecountColorMap {
    boundaries: Vec<(i16, Color32)>,
}
impl MinecountColorMap {
    pub fn new(mut rules: Vec<(i16, Color32)>, fallback: Color32) -> Self {
        rules.push((i16::MAX, fallback));
        rules.sort_by(|a, b| a.0.cmp(&b.0));
        Self { boundaries: rules }
    }
    pub fn get(&self, minecount: i16) -> Color32 {
        self.boundaries.iter().find(|i| minecount <= i.0).unwrap().1
    }
    pub fn rules(&self) -> &[(i16, Color32)] {
        self.boundaries.as_slice().split_first().unwrap().1
    }
    pub fn fallback(&self) -> Color32 {
        self.boundaries.first().unwrap().1
    }
}

impl Default for MinecountColorMap {
    fn default() -> Self {
        Self::new(
            vec![
                (-1, Color32::PURPLE),
                (0, Color32::DARK_GRAY),
                (1, Color32::DARK_GREEN),
                (2, Color32::GREEN),
                (4, Color32::YELLOW),
                (6, Color32::ORANGE),
                (10, Color32::RED),
            ],
            Color32::DARK_RED,
        )
    }
}
