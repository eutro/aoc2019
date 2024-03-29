use std::ops::{Add, Sub};
use std::str::FromStr;

pub static DIRECTIONS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    pub fn offset<T: From<u8> + Add<Output = T> + Sub<Output = T>>(&self, pos: (T, T)) -> (T, T) {
        self.offset_by(pos, 1.into())
    }

    pub fn offset_by<T: Add<Output = T> + Sub<Output = T>>(&self, (x, y): (T, T), by: T) -> (T, T) {
        match self {
            Dir::North => (x, y + by),
            Dir::South => (x, y - by),
            Dir::West => (x - by, y),
            Dir::East => (x + by, y),
        }
    }

    pub fn turns(&self) -> [Dir; 2] {
        match self {
            Dir::North => [Dir::West, Dir::East],
            Dir::South => [Dir::East, Dir::West],
            Dir::West => [Dir::North, Dir::South],
            Dir::East => [Dir::South, Dir::North],
        }
    }

    pub fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Dir::North => "north",
            Dir::South => "south",
            Dir::West => "west",
            Dir::East => "east",
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "north" => Dir::North,
            "south" => Dir::South,
            "west" => Dir::West,
            "east" => Dir::East,
            _ => return Err(()),
        })
    }
}
