use std::ops::{Add, Sub};

pub static DIRECTIONS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

#[derive(Clone, Copy)]
pub enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    pub fn offset<T: From<u8> + Add<Output=T> + Sub<Output=T>>(&self, (x, y): (T, T)) -> (T, T) {
        match self {
            Dir::North => (x, y + 1.into()),
            Dir::South => (x, y - 1.into()),
            Dir::West => (x - 1.into(), y),
            Dir::East => (x + 1.into(), y),
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
}
