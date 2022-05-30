use crate::io::{self, stdin, BufRead};
use crate::util::{Dir, DIRECTIONS};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::Index;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum ParsedTile {
    Wall,
    Space,
    None,
    HalfPortal(char),
}

impl From<char> for ParsedTile {
    fn from(c: char) -> Self {
        match c {
            '.' => ParsedTile::Space,
            '#' => ParsedTile::Wall,
            ' ' => ParsedTile::None,
            c => ParsedTile::HalfPortal(c),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Portal(char, char);

impl Display for Portal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Side {
    Inner,
    Outer,
}

impl Side {
    fn deepen(&self, depth: usize) -> Option<usize> {
        match self {
            Side::Outer => {
                if depth == 0 {
                    None
                } else {
                    Some(depth - 1)
                }
            }
            Side::Inner => Some(depth + 1),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Wall,
    Space,
    None,
    Unpaired(Portal),
    Paired(Side, Portal, (usize, usize)),
    HalfPortal(char),
}

impl Tile {
    fn traversible(&self) -> bool {
        match self {
            Tile::Space => true,
            Tile::Paired(_, _, _) => true,
            Tile::Unpaired(_) => true,
            Tile::Wall => false,
            Tile::None => false,
            Tile::HalfPortal(_) => false,
        }
    }

    fn neighbours(
        &self,
        depth: usize,
        pos: (usize, usize),
    ) -> impl Iterator<Item = (usize, (usize, usize))> {
        DIRECTIONS
            .iter()
            .map(move |&d| (depth, d.offset(pos)))
            .chain(self.linked(depth).into_iter())
    }

    fn linked(&self, depth: usize) -> Option<(usize, (usize, usize))> {
        match self {
            Tile::Paired(side, _, link) => side.deepen(depth).map(|d| (d, *link)),
            _ => None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Space => '.',
                Tile::None => ' ',
                Tile::Unpaired(_) => '*',
                Tile::Paired(Side::Outer, _, _) => 'o',
                Tile::Paired(Side::Inner, _, _) => 'i',
                Tile::HalfPortal(c) => *c,
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Donut {
    tiles: Vec<Vec<Tile>>,
}

impl Donut {
    fn iter(&self) -> impl Iterator<Item = ((usize, usize), Tile)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, &t)| ((x, y), t)))
    }
}

impl Index<(usize, usize)> for Donut {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[y][x]
    }
}

impl From<Vec<Vec<ParsedTile>>> for Donut {
    fn from(v: Vec<Vec<ParsedTile>>) -> Self {
        let mut linked = Vec::new();
        let mut unlinked: HashMap<Portal, (usize, usize)> = HashMap::new();
        let mut max_x = usize::MIN;
        let mut tiles = v
            .iter()
            .enumerate()
            .map(|(y, line)| {
                max_x = max_x.max(line.len());
                line.iter()
                    .enumerate()
                    .map(|(x, &tile)| match tile {
                        ParsedTile::Wall => Tile::Wall,
                        ParsedTile::Space => Tile::Space,
                        ParsedTile::None => Tile::None,
                        ParsedTile::HalfPortal(c) => {
                            if x > 0 && y > 0 {
                                let mut full = None;
                                let mut open = None;
                                for d in &DIRECTIONS {
                                    let (tx, ty) = d.offset((x, y));
                                    match v.get(ty).and_then(|p| p.get(tx)) {
                                        None => {}
                                        Some(&t) => match t {
                                            ParsedTile::HalfPortal(oc) => {
                                                full = Some(match d {
                                                    Dir::North => Portal(c, oc),
                                                    Dir::South => Portal(oc, c),
                                                    Dir::West => Portal(oc, c),
                                                    Dir::East => Portal(c, oc),
                                                });
                                            }
                                            ParsedTile::Space => {
                                                open = Some((tx, ty));
                                            }
                                            _ => {}
                                        },
                                    }
                                }
                                if open.is_some() {
                                    let pos = open.unwrap();
                                    let portal = full.unwrap();
                                    match unlinked.remove(&portal) {
                                        Some(other) => {
                                            linked.push((portal, other, pos));
                                        }
                                        None => {
                                            unlinked.insert(portal, pos);
                                        }
                                    }
                                }
                            }
                            Tile::HalfPortal(c)
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        for (portal, (x, y)) in unlinked {
            tiles[y][x] = Tile::Unpaired(portal);
        }
        let max_y = v.len();
        let side = |x: usize, y: usize| {
            const OUT_WIDTH: usize = 3;
            if x <= OUT_WIDTH || y <= OUT_WIDTH || x >= max_x - OUT_WIDTH || y >= max_y - OUT_WIDTH
            {
                Side::Outer
            } else {
                Side::Inner
            }
        };
        for (portal, (ax, ay), (bx, by)) in linked {
            tiles[ay][ax] = Tile::Paired(side(ax, ay), portal, (bx, by));
            tiles[by][bx] = Tile::Paired(side(bx, by), portal, (ax, ay));
        }
        Donut { tiles }
    }
}

impl Display for Donut {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.tiles {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn traverse_donut(start: (usize, usize), donut: &Donut, with_depth: bool) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut tq = VecDeque::new();
    q.push_back((0usize, start));
    let mut seen = HashSet::new();
    seen.insert((0usize, start));
    let mut steps = 0;
    while !q.is_empty() {
        while !q.is_empty() {
            let (depth, pos) = q.pop_front().unwrap();
            let tile = donut[pos];
            if matches!(tile, Tile::Unpaired(Portal('Z', 'Z'))) && depth == 0 {
                return Some(steps);
            }
            for (prov_depth, neighbour) in tile.neighbours(if with_depth { depth } else { 1 }, pos)
            {
                let neighbour_tile = donut[neighbour];
                let new_depth = if with_depth { prov_depth } else { 0 };
                if neighbour_tile.traversible() {
                    // going through seen inner portals could also be filtered out here but it's fast enough anyway
                    if seen.insert((new_depth, neighbour)) {
                        tq.push_back((new_depth, neighbour));
                    }
                }
            }
        }
        steps += 1;
        swap(&mut q, &mut tq);
    }
    None
}

#[no_mangle]
pub fn day_20() {
    let stdin = stdin();
    let donut: Donut = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().map(ParsedTile::from).collect_vec())
        .collect_vec()
        .into();

    let start = donut
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Unpaired(Portal('A', 'A'))))
        .unwrap()
        .0;

    io::println!("Steps: {}", traverse_donut(start, &donut, false).unwrap());
    io::println!("Steps: {}", traverse_donut(start, &donut, true).unwrap());
}
