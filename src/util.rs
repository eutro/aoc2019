pub static DIRECTIONS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

#[derive(Clone, Copy)]
pub enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    pub fn offset(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::North => (x, y + 1),
            Dir::South => (x, y - 1),
            Dir::West => (x - 1, y),
            Dir::East => (x + 1, y),
        }
    }
}
