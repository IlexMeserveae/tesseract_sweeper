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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ordinate { X, Y, Z, W }