use std::cmp::{max, min};
use std::ops::Range;
use rand::random;
use coordinate::{Coordinate, Ordinate};
use tile::Tile;

#[cfg(test)]
mod tests;

pub mod coordinate;
mod tile;

pub struct Minefield {
    size: Coordinate, tiles: Vec<Tile>, mines_remaining: i16, delta: bool
}
impl Minefield {
    pub fn new(size: Coordinate, mine_count: u16) -> Result<Self, String> {
        let mut tiles = vec![false; size.x() * size.y() * size.z() * size.w()];
        if tiles.len() == 0 { return Err("Minefield dimensions cannot be zero.".to_string()); }
        if tiles.len() < mine_count.into() { return Err(
            String::from("Mine count is too large for the given board dimensions."))}

        let mut mine_positions = vec![];
        let mut count = mine_count;
        while count > 0 {
            let index = random::<u32>() as usize % tiles.len();
            if tiles[index] { continue; }
            tiles[index] = true;
            mine_positions.push(index);
            count -= 1;
        }
        let tiles = tiles.into_iter().map(|mine| Tile::new(mine)).collect();

        let mut field = Self { size, tiles,
            mines_remaining: mine_count as i16, delta: true };

        for index in mine_positions {
            let coord = field.convert_index(index);
            for neighbour in field.get_neighbours(coord, 1) {
                field.index_mut(neighbour).count_mine();
            };
        };

        Ok(field)
    }
    pub fn quickstart(&mut self) -> Result<(), String> {
        let mut count = 0;
        while count < 10000 {
            let index = random::<u32>() as usize % self.tiles.len();
            if self.tiles[index].minecount() > 0 { count += 1; continue; }
            self.reveal(self.convert_index(index))?;
            return Ok(())
        };

        Err("Cannot find tile without adjacent mines!".into())
    }

    pub(super) fn iter_ordinate(&self, coord: Coordinate, radius: usize, ordinate: Ordinate) -> Range<usize> {
        let val = coord.get_ordinate(ordinate);
        let len = self.length(ordinate);
        (max(val, radius + 1) - radius)..min(val + radius + 1, len + 1)
    }
    pub fn get_neighbours(&self, coord: Coordinate, radius: usize) -> Vec<Coordinate> {
        let mut neighbours = Vec::new();
        for x in self.iter_ordinate(coord, radius, Ordinate::X) {
            for y in self.iter_ordinate(coord, radius, Ordinate::Y) {
                for z in self.iter_ordinate(coord, radius, Ordinate::Z) {
                    for w in self.iter_ordinate(coord, radius, Ordinate::W) {
                        let coord_2 = coordinate::coordinate(x, y, z, w);
                        if coord == coord_2 { continue; }
                        neighbours.push(coord_2);
                    };
                };
            };
        };
        neighbours
    }

    fn convert_index(&self, mut index: usize) -> Coordinate {
        let x = 1 + index % self.size.x(); index /= self.size.x();
        let y = 1 + index % self.size.y(); index /= self.size.y();
        let z = 1 + index % self.size.z(); index /= self.size.z();
        let w = 1 + index % self.size.w();
        coordinate::coordinate(x, y, z, w)
    }
    fn convert_coord(&self, coord: Coordinate) -> usize {
        (coord.x() - 1) + (coord.y() - 1) * self.size.x() +
        ((coord.z() - 1) + (coord.w()- 1) * self.size.z()) * (self.size.x() * self.size.y())
    }
    pub fn index(&self, coord: Coordinate) -> &Tile {
        let index = self.convert_coord(coord);
        &self.tiles[index]
    }
    pub fn index_mut(&mut self, coord: Coordinate) -> &mut Tile {
        let index = self.convert_coord(coord);
        &mut self.tiles[index]
    }

    pub fn toggle_delta(&mut self) { self.delta = !self.delta; }
    pub fn toggle_flagged(&mut self, coord: Coordinate) -> bool {
        let tile = self.index_mut(coord);
        tile.toggle_flagged();
        let flagged = tile.is_flagged();

        for neighbour in self.get_neighbours(coord, 1) {
            if flagged { self.index_mut(neighbour).increase_flagged_minecount() }
            else { self.index_mut(neighbour).decrease_flagged_minecount() }.unwrap()
        };
        self.mines_remaining -= 1;
        flagged
    }
    pub fn reveal(&mut self, coord: Coordinate) -> Result<(), String> {
        let err_msg = || "There was a mine there!".to_string();

        self.index_mut(coord).reveal();
        match self.query_tile(coord) {
            QueryResult::Exploded => return Err(err_msg()),
            QueryResult::Revealed(0) => {},
            QueryResult::Revealed(_) => return Ok(()),
            _ => unreachable!()
        }

        // Chain reveal
        let mut stack = vec![coord];
        while let Some(coord) = stack.pop() {
            for neighbour in self.get_neighbours(coord, 1) {
                let tile = self.index(neighbour);
                if tile.is_revealed() || tile.is_flagged() { continue; }
                self.index_mut(neighbour).reveal();
                match self.query_tile(neighbour) {
                    QueryResult::Exploded => return Err(err_msg()),
                    QueryResult::Revealed(0) => stack.push(neighbour),
                    QueryResult::Revealed(_) => {},
                    _ => unreachable!()
                }
            }
        };

        Ok(())
    }

    pub fn delta(&self) -> bool { self.delta }
    pub fn mines_remaining(&self) -> i16 { self.mines_remaining }
    pub fn length(&self, ordinate: Ordinate) -> usize { self.size.get_ordinate(ordinate) }
    pub fn query_tile(&self, coord: Coordinate) -> QueryResult {
        let tile = self.index(coord);

        if tile.is_flagged() { return QR::Flagged; }
        if !tile.is_revealed() { return QR::Blank; }
        if tile.has_mine() { return QR::Exploded; }

        let count = if self.delta { tile.delta_minecount() } else { tile.minecount() as i16 };
        QR::Revealed(count)
    }
    pub fn query_tile_gameover(&self, coord: Coordinate) -> QueryResult {
        let tile = self.index(coord);
        let count = if self.delta { tile.delta_minecount() } else { tile.minecount() as i16 };

        if tile.is_flagged() { 
            return if tile.has_mine() { QR::GoCorrect } else { QR::GoIncorrect } 
        }
        if !tile.is_revealed() { 
            return if tile.has_mine() { QR::GoMine } else { QR::GoUnrevealed(count) } 
        }
        if tile.has_mine() { 
            return QR::Exploded 
        }
        QR::Revealed(count)
    }
}

type QR = QueryResult;
pub enum QueryResult {
    Blank,
    Flagged,
    Revealed(i16),
    Exploded,
    // Game Over Exclusive
    GoMine,
    GoCorrect,
    GoIncorrect,
    GoUnrevealed(i16),
}

