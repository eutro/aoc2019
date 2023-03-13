use crate::io;
use crate::io::{stdin, BufRead};
use crate::util::DIRECTIONS;
use itertools::Itertools;
use itertools::__std_iter::FromIterator;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

const SIZE: i16 = 5;

trait Pos: Hash + Eq + Sized {
    fn neighbours(&self) -> Vec<Self>;
}

#[derive(Hash, Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
struct FlatPos {
    x: i16,
    y: i16,
}

impl Pos for FlatPos {
    fn neighbours(&self) -> Vec<Self> {
        DIRECTIONS
            .iter()
            .map(|&d| d.offset((self.x, self.y)))
            .filter(|(x, y)| *x >= 0 && *x < SIZE && *y >= 0 && *y < SIZE)
            .map(|(x, y)| FlatPos { x, y })
            .collect_vec()
    }
}

impl FlatPos {
    fn biodiversity(&self) -> u32 {
        1 << self.y * SIZE + self.x
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct RecPos {
    x: i16,
    y: i16,
    depth: i32,
}

impl Pos for RecPos {
    fn neighbours(&self) -> Vec<Self> {
        let depth = self.depth;
        DIRECTIONS
            .iter()
            .map(|&d| (d, d.offset((self.x, self.y))))
            .flat_map(|(d, (x, y))| {
                (if x < 0 || y < 0 || x >= SIZE || y >= SIZE {
                    let (x, y) = d.offset((SIZE / 2, SIZE / 2));
                    vec![RecPos {
                        x,
                        y,
                        depth: depth - 1,
                    }]
                } else if x == SIZE / 2 && y == SIZE / 2 {
                    let (dx, dy) = d.opposite().offset_by((SIZE / 2, SIZE / 2), SIZE / 2);
                    let mut v = Vec::new();
                    v.push((dx, dy));
                    for &t in &d.turns() {
                        v.push(t.offset((dx, dy)));
                        v.push(t.offset_by((dx, dy), 2));
                    }
                    v.into_iter()
                        .map(|(x, y)| RecPos {
                            x,
                            y,
                            depth: depth + 1,
                        })
                        .collect()
                } else {
                    vec![RecPos { x, y, depth }]
                })
                .into_iter()
            })
            .collect_vec()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Eris<T: Pos> {
    live: HashSet<T>,
}

impl<T: Pos> Eris<T> {
    fn new(live: HashSet<T>) -> Self {
        Eris { live }
    }

    fn live(&self) -> impl Iterator<Item = &T> {
        self.live.iter()
    }

    #[allow(unused)]
    fn alive(&self, pos: T) -> bool {
        self.live.contains(&pos)
    }

    fn next(&self) -> Self {
        let mut neighbour_counts = HashMap::new();
        for pos in &self.live {
            for n in pos.neighbours() {
                if let Some(v) = neighbour_counts.get_mut(&n) {
                    *v += 1;
                } else {
                    neighbour_counts.insert(n, 1usize);
                }
            }
        }
        Eris {
            live: neighbour_counts
                .into_iter()
                .filter(|(pos, count)| *count == 1 || (*count == 2 && !self.live.contains(pos)))
                .map(|(pos, _)| pos)
                .collect(),
        }
    }
}

impl<T: Pos + Ord> Hash for Eris<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for live in self.live.iter().sorted() {
            live.hash(state);
        }
    }
}

impl<P: Pos> FromIterator<P> for Eris<P> {
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Eris::new(iter.into_iter().collect())
    }
}

impl Display for Eris<FlatPos> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            (0..SIZE)
                .map(|y| (0..SIZE)
                    .map(|x| {
                        if self.alive(FlatPos { x, y }) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .join(""))
                .join("\n")
        )
    }
}

impl Display for Eris<RecPos> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for depth in self.live().map(|p| p.depth).unique().sorted() {
            write!(
                f,
                "Depth {}:\n{}",
                depth,
                (0..SIZE)
                    .map(|y| (0..SIZE)
                        .map(|x| {
                            if x == SIZE / 2 && y == SIZE / 2 {
                                '?'
                            } else if self.alive(RecPos { x, y, depth }) {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .join(""))
                    .join("\n\n")
            )?;
        }
        Ok(())
    }
}

const MINUTES: i32 = 200;

#[no_mangle]
pub fn day_24() {
    let stdin = stdin();
    let start = stdin
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i16, y as i16))
                .collect_vec()
                .into_iter()
        })
        .collect_vec();

    let mut eris = start
        .iter()
        .map(|&(x, y)| FlatPos { x, y })
        .collect::<Eris<_>>();
    let mut erises = HashSet::new();
    loop {
        erises.insert(eris.clone());
        eris = eris.next();
        if erises.contains(&eris) {
            break;
        }
    }

    let biodiversity = eris.live().map(FlatPos::biodiversity).sum::<u32>();
    io::println!("Biodiversity: {}", biodiversity);

    let mut eris = start
        .iter()
        .map(|&(x, y)| RecPos { x, y, depth: 0 })
        .collect::<Eris<_>>();
    for _ in 0..MINUTES {
        eris = eris.next();
    }
    io::println!("Bugs: {}", eris.live().count());
}


