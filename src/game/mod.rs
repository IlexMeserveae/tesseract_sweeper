use std::cmp::{max, min};
use std::ops::{Range};
use rand::random;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ordinate { X, Y, Z, W }

pub struct Minefield {
    size: Coordinate, tiles: Vec<Tile>, mines_remaining: i16, delta: bool
}

impl Minefield {
    pub fn new(size: Coordinate, mine_count: u16) -> Result<Self, String> {
        let mut tiles = vec![false; size.x * size.y * size.z * size.w];
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
                field.index_mut(neighbour).minecount += 1;
            };
        };

        Ok(field)
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
                        let coord_2 = coordinate(x, y, z, w);
                        if coord == coord_2 { continue; }
                        neighbours.push(coord_2);
                    };
                };
            };
        };
        neighbours
    }

    fn convert_index(&self, mut index: usize) -> Coordinate {
        let x = 1 + index % self.size.x; index /= self.size.x;
        let y = 1 + index % self.size.y; index /= self.size.y;
        let z = 1 + index % self.size.z; index /= self.size.z;
        let w = 1 + index % self.size.w;
        coordinate(x, y, z, w)
    }
    fn convert_coord(&self, coord: Coordinate) -> usize {
        (coord.x - 1) + (coord.y - 1) * self.size.x +
        ((coord.z - 1) + (coord.w- 1) * self.size.z) * (self.size.x * self.size.y)
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
            QueryResult::Mine => return Err(err_msg()),
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
                    QueryResult::Mine => return Err(err_msg()),
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

        if tile.is_flagged() { return QueryResult::Flagged; }
        if !tile.is_revealed() { return QueryResult::Blank; }
        if tile.has_mine() { return QueryResult::Mine; }

        let count = if self.delta { tile.delta_minecount() } else { tile.minecount() as i16 };
        QueryResult::Revealed(count)
    }
}

pub enum QueryResult {
    Blank,
    Flagged,
    Revealed(i16),
    Mine,
}

pub fn coordinate(x: usize, y: usize, z: usize, w: usize) -> Coordinate {
    Coordinate::new(x , y, z, w).unwrap()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coordinate {
    x: usize, y: usize, z: usize, w: usize
}

impl Coordinate {
    pub fn new(x: usize, y: usize, z: usize, w: usize) -> Result<Self, String> {
        if x == 0 || y == 0 || z == 0 || w == 0 {
            return Err(String::from("Ordinates must be positive!"))
        }

        Ok(Coordinate { x, y, z, w })
    }
    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }
    pub fn z(&self) -> usize { self.z }
    pub fn w(&self) -> usize { self.w }
    pub fn get_ordinate(&self, ordinate: Ordinate) -> usize {
        match ordinate {
            Ordinate::X => self.x,
            Ordinate::Y => self.y,
            Ordinate::Z => self.z,
            Ordinate::W => self.w,
        }
    }
    pub fn get_xy(&self) -> (usize, usize) { (self.x, self.y) }
    pub fn get_zw(&self) -> (usize, usize) { (self.z, self.w) }
}

pub struct Tile {
    has_mine: bool, is_flagged: bool, is_revealed: bool,
    minecount: u16, flagged_minecount: u16,
}

impl Tile {
    pub fn new(has_mine: bool) -> Self {
        Self { has_mine, is_flagged: false, is_revealed: false,
            minecount: 0, flagged_minecount: 0 }
    }

    pub fn has_mine(&self) -> bool { self.has_mine }
    pub fn minecount(&self) -> u16 { self.minecount }
    pub fn delta_minecount(&self) -> i16 { self.minecount as i16 - self.flagged_minecount as i16 }
    pub fn is_flagged(&self) -> bool { self.is_flagged }
    pub fn is_revealed(&self) -> bool { self.is_revealed }

    pub(super) fn decrease_flagged_minecount(&mut self) -> Result<(), String> {
        if self.flagged_minecount == 0 {
            return Err("Cannot decrease flagged minecount below 0.".into());
        }
        self.flagged_minecount -= 1;
        Ok(())
    }
    pub(super) fn increase_flagged_minecount(&mut self) -> Result<(), String> {
        // if self.minecount == self.flagged_minecount {
        //     return Err("Cannot increase flagged minecount above total minecount.".into());
        // }
        self.flagged_minecount += 1;
        Ok(())
    }
    pub(super) fn toggle_flagged(&mut self) {
        self.is_flagged = !(self.is_flagged || self.is_revealed);
    }
    pub(super) fn reveal(&mut self) { self.is_revealed = true; self.is_flagged = false }
}