use eframe::egui::{vec2, Vec2};

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