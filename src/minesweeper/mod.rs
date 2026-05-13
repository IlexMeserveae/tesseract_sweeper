use std::cmp::{max, min};
use std::collections::VecDeque;
use std::ops::Range;
use rand::random;
use coordinate::{Coordinate, Ordinate};
use tile::Tile;
use crate::minesweeper::tile::{TileError, TileResult};

#[cfg(test)]
mod tests;

pub mod coordinate;
pub mod tile;

pub struct Minefield {
    size: Coordinate, tiles: Vec<Tile>, 
    total_mines: i16, flagged_mines: i16, blanks_remaining: i16, 
    delta: bool
}
impl Minefield {
    fn neighbour_no(field_size: Coordinate, index: usize) -> u16 {
        let mut tiles = 81;
        let coord = Self::convert_index(field_size, index);
        for ord in [Ordinate::X, Ordinate::Y, Ordinate::Z, Ordinate::W] {
            let size_ord = field_size.get(ord);
            let coord_ord = coord.get(ord);
            if coord_ord == 1 || coord_ord == size_ord { tiles *= 2; tiles /= 3; }
            if size_ord == 1 { tiles /= 2; }
        };
        tiles - 1
    }

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
        let tiles = tiles.into_iter().enumerate().map(|(i, mine)|
            Tile::new(mine, Self::neighbour_no(size, i))).collect();

        let mut field = Self { size, tiles, total_mines: mine_count as i16, 
            flagged_mines: 0, blanks_remaining: size.multiply_out() as i16, delta: true };

        for index in mine_positions {
            let coord = Self::convert_index(size, index);
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
            if self.tiles[index].true_minecount() > 0 { count += 1; continue; }
            if self.tiles[index].has_mine() { continue}
            self.reveal(Self::convert_index(self.size, index)).expect("Not Possible!");
            return Ok(())
        };

        Err("Cannot find tile without adjacent mines!".into())
    }

    pub(super) fn iter_ordinate(&self, coord: Coordinate, radius: usize, ordinate: Ordinate) -> Range<usize> {
        let val = coord.get(ordinate);
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

    fn convert_index(size: Coordinate, mut index: usize) -> Coordinate {
        let x = 1 + index % size.x(); index /= size.x();
        let y = 1 + index % size.y(); index /= size.y();
        let z = 1 + index % size.z(); index /= size.z();
        let w = 1 + index % size.w();
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
        self.flagged_mines += if flagged { 1 } else { -1 };
        flagged
    }

    fn reveal_inner(&mut self, coord: Coordinate) -> Result<u16, TileError> {
        let mut revealed = 0;

        let mut stack = vec![coord];
        while let Some(coord) = stack.pop() {
            revealed += 1;
            let e = self.index_mut(coord).reveal();
            e.expect("Failed to reveal!");

            let is_zero = QR::Revealed(0) == self.query_tile(coord);
            for next in self.get_neighbours(coord, 1) {
                let tile = self.index_mut(next);
                tile.decrease_hidden_neighbours()?;

                if tile.is_revealed() { continue; }
                if tile.is_flagged() || !is_zero || stack.contains(&next) { continue; }
                stack.push(next);
            }
        };

        Ok(revealed)
    }
    pub fn reveal(&mut self, coord: Coordinate) -> TileResult {
        self.reveal_inner(coord)?;
        if !self.delta { return Ok(()); }

        let mut zeros = vec![];
        let mut zero_queue = VecDeque::from(vec![coord]);
        while let Some(coord) = zero_queue.pop_front() {
            for neighbour in self.get_neighbours(coord, 1) {
                if zeros.contains(&neighbour) { continue; }
                match self.query_tile(neighbour) {
                    QR::Revealed(0) => {
                        zeros.push(neighbour);
                        zero_queue.push_back(neighbour);
                    },
                    _ => {}
                }
            }
        }

        // // Debug
        // for &z in &zeros {
        //     self.index_mut(z).toggle_debug_mark();
        // }

        for zero in zeros {
            for neighbour in self.get_neighbours(zero, 1) {
                let tile = self.index_mut(neighbour);
                if tile.is_revealed() || tile.is_flagged() { continue; }
                self.reveal_inner(neighbour)?;
            }
        };

        Ok(())
    }

    pub fn delta(&self) -> bool { self.delta }
    pub fn mines_remaining(&self) -> i16 { self.total_mines - self.flagged_mines }
    pub fn length(&self, ordinate: Ordinate) -> usize { self.size.get(ordinate) }
    pub fn query_tile(&self, coord: Coordinate) -> QueryResult {
        let tile = self.index(coord);
        let count = tile.minecount(self.delta);

        // Debug
        if tile.is_debug_marked() { return QueryResult::DebugMarked; }

        if tile.is_flagged() { return QR::Flagged; }
        if !tile.is_revealed() { return QR::Hidden; }
        if tile.has_mine() { return QR::Exploded; }
        if tile.is_blank(self.delta) { return QR::Blank; }

        QR::Revealed(count)
    }
    pub fn query_tile_gameover(&self, coord: Coordinate) -> QueryResult {
        let tile = self.index(coord);
        let count = tile.minecount(self.delta);

        if tile.is_flagged() { 
            return if tile.has_mine() { QR::GoCorrect } else { QR::GoIncorrect } 
        }
        if !tile.is_revealed() { 
            return if tile.has_mine() { QR::GoMine } else { QR::GoHidden(count) } 
        }
        if tile.has_mine() { 
            return QR::Exploded 
        }
        if tile.is_blank(self.delta) {
            return QR::Blank;
        }

        QR::Revealed(count)
    }
}


type QR = QueryResult;
#[derive(PartialEq)]
pub enum QueryResult {
    Hidden,
    Flagged,
    Revealed(i16),
    Blank,
    Exploded,
    // Game Over Exclusive
    GoMine,
    GoCorrect,
    GoIncorrect,
    GoHidden(i16),
    // Debug Exclusive
    DebugMarked,
}

