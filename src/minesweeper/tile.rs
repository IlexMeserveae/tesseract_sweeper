pub struct Tile {
    has_mine: bool, is_flagged: bool, is_revealed: bool,
    minecount: u16, flagged_minecount: u16,
}

impl Tile {
    pub fn new(has_mine: bool) -> Self {
        Self { has_mine, is_flagged: false, is_revealed: false,
            minecount: 0, flagged_minecount: 0 }
    }
    
    pub(super) fn count_mine(&mut self) { self.minecount += 1 }
    pub fn has_mine(&self) -> bool { self.has_mine }
    pub fn minecount(&self) -> u16 { self.minecount }
    pub fn delta_minecount(&self) -> i16 { self.minecount as i16 - self.flagged_minecount as i16 }
    pub fn is_flagged(&self) -> bool { self.is_flagged }
    pub fn is_revealed(&self) -> bool { self.is_revealed }

    pub(crate) fn decrease_flagged_minecount(&mut self) -> Result<(), String> {
        if self.flagged_minecount == 0 {
            return Err("Cannot decrease flagged minecount below 0.".into());
        }
        self.flagged_minecount -= 1;
        Ok(())
    }
    pub(crate) fn increase_flagged_minecount(&mut self) -> Result<(), String> {
        // if self.minecount == self.flagged_minecount {
        //     return Err("Cannot increase flagged minecount above total minecount.".into());
        // }
        self.flagged_minecount += 1;
        Ok(())
    }
    pub(crate) fn toggle_flagged(&mut self) {
        self.is_flagged = !(self.is_flagged || self.is_revealed);
    }
    pub(crate) fn reveal(&mut self) { self.is_revealed = true; self.is_flagged = false }
}