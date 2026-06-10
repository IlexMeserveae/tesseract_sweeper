use std::fmt::Display;

pub fn coordinate(x: usize, y: usize, z: usize, w: usize) -> Coordinate {
    Coordinate::new(x , y, z, w).unwrap()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coordinate {
    x: usize, y: usize, z: usize, w: usize
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
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
    pub fn get(&self, ordinate: Ordinate) -> usize {
        match ordinate {
            Ordinate::X => self.x,
            Ordinate::Y => self.y,
            Ordinate::Z => self.z,
            Ordinate::W => self.w,
        }
    }
    pub fn get_xy(&self) -> (usize, usize) { (self.x, self.y) }
    pub fn get_zw(&self) -> (usize, usize) { (self.z, self.w) }
    pub fn multiply_out(&self) -> usize { self.x * self.y * self.w * self.z }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Ordinate { X, Y, Z, W }
impl Ordinate {
    pub fn name(&self) -> &'static str { match self {
        Ordinate::X => "x",
        Ordinate::Y => "y",
        Ordinate::Z => "z",
        Ordinate::W => "w"
    }}
}
impl Display for Ordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name().to_uppercase())
    }
}

pub const ORDINATES: [Ordinate; 4] = [Ordinate::X, Ordinate::Y, Ordinate::Z, Ordinate::W];