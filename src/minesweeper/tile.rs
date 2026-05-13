use std::fmt::{Debug, Display, Formatter};
use crate::minesweeper::tile::TileError::{Exploded, Underflow, Overflow, AlreadyRevealed};

pub struct Tile {
    has_mine: bool, is_flagged: bool, is_revealed: bool,
    true_minecount: u16, flagged_neighbours: u16, hidden_neighbours: u16,
    
    // Debug
    debug_marked: bool,
}

pub type TileResult = Result<(), TileError>;
pub enum TileError {
    Exploded,
    AlreadyRevealed,
    Overflow(String),
    Underflow(String),
}
impl Debug for TileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Exploded => write!(f, "There was a mine there!"),
            AlreadyRevealed => write!(f, "The tile is already revealed!"),
            Underflow(msg) => write!(f, "{}", msg),
            Overflow(msg) => write!(f, "{}", msg),
        }
    }
}
impl Display for TileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Tile {
    pub fn new(has_mine: bool, neighbours: u16) -> Self {
        Self { has_mine, is_flagged: false, is_revealed: false,
            true_minecount: 0, flagged_neighbours: 0, hidden_neighbours: neighbours,
            debug_marked: false }
    }
    
    pub(super) fn count_mine(&mut self) {
        self.true_minecount += 1
    }
    pub fn is_blank(&self, delta: bool) -> bool {
        self.is_revealed && self.hidden_neighbours == self.flagged_neighbours
            && self.minecount(delta) == 0
    }
    pub fn has_mine(&self) -> bool {
        self.has_mine
    }
    pub fn true_minecount(&self) -> u16 {
        self.true_minecount
    }
    pub fn minecount(&self, delta: bool) -> i16 {
        if delta { self.true_minecount as i16 - self.flagged_neighbours as i16 } 
        else { self.true_minecount as i16 }
    }
    pub fn hidden_neighbours(&self) -> u16 { self.hidden_neighbours }
    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }
    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub(crate) fn decrease_flagged_minecount(&mut self) -> TileResult {
        if self.flagged_neighbours == 0 {
            return Err(Underflow("Cannot decrease flagged minecount below 0.".into()));
        }
        self.flagged_neighbours -= 1;
        Ok(())
    }
    pub(crate) fn increase_flagged_minecount(&mut self) -> TileResult {
        // if self.minecount == self.flagged_minecount {
        //     return Err("Cannot increase flagged minecount above total minecount.".into());
        // }
        self.flagged_neighbours += 1;
        Ok(())
    }
    pub(crate) fn decrease_hidden_neighbours(&mut self) -> TileResult {
        if self.hidden_neighbours == 0 {
            return Err(Underflow("Cannot decrease hidden neighbours below 0.".into()));
        }
        self.hidden_neighbours -= 1;
        Ok(())
    }
    pub(crate) fn toggle_flagged(&mut self) {
        self.is_flagged = !(self.is_flagged || self.is_revealed);
    }
    pub(crate) fn reveal(&mut self) -> TileResult {
        if self.is_revealed { return Err(AlreadyRevealed) }
        self.is_revealed = true;
        self.is_flagged = false;
        if self.has_mine { Err(Exploded) } else { Ok(()) }
    }
    
    // Debug
    pub fn toggle_debug_mark(&mut self) { self.debug_marked = !self.debug_marked; }
    pub fn is_debug_marked(&self) -> bool { self.debug_marked }
    
}