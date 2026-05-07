pub struct Tile {
    has_mine: bool, is_flagged: bool, is_revealed: bool,
    minecount: u16, flagged_neighbours: u16, hidden_neighbours: Option<u16>,
}

impl Tile {
    pub fn new(has_mine: bool) -> Self {
        Self { has_mine, is_flagged: false, is_revealed: false,
            minecount: 0, flagged_neighbours: 0, hidden_neighbours: None}
    }
    
    pub(super) fn count_mine(&mut self) {
        self.minecount += 1
    }
    pub(super) fn set_hidden_count(&mut self, count: u16) {
        self.hidden_neighbours = Some(count)
    }
    pub fn is_blank(&self) -> bool {
        self.is_revealed && self.hidden_neighbours == Some(self.flagged_neighbours)
    }
    pub fn has_mine(&self) -> bool {
        self.has_mine
    }
    pub fn minecount(&self) -> u16 {
        self.minecount
    }
    pub fn delta_minecount(&self) -> i16 {
        self.minecount as i16 - self.flagged_neighbours as i16
    }
    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }
    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    pub(crate) fn decrease_flagged_minecount(&mut self) -> Result<(), String> {
        if self.flagged_neighbours == 0 {
            return Err("Cannot decrease flagged minecount below 0.".into());
        }
        self.flagged_neighbours -= 1;
        Ok(())
    }
    pub(crate) fn increase_flagged_minecount(&mut self) -> Result<(), String> {
        // if self.minecount == self.flagged_minecount {
        //     return Err("Cannot increase flagged minecount above total minecount.".into());
        // }
        self.flagged_neighbours += 1;
        Ok(())
    }
    pub(crate) fn decrease_hidden_neighbours(&mut self) {
        if let Some(n) = self.hidden_neighbours { self.hidden_neighbours = Some(n - 1); }
    }
    pub(crate) fn toggle_flagged(&mut self) {
        self.is_flagged = !(self.is_flagged || self.is_revealed);
    }
    pub(crate) fn reveal(&mut self) -> Result<(), String> {
        self.is_revealed = true;
        self.is_flagged = false;
        if self.has_mine { Err("There was a mine there!".into()) } else { Ok(()) }
    }
}