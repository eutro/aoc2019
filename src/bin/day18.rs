use crate::io;
use crate::io::{stdin, BufRead};
use crate::util::DIRECTIONS;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    StoneWall,
    OpenPassage,
    Entrance,
    Key(char),
    Door(char),
}

impl Tile {
    fn of(c: char) -> Self {
        match c {
            '#' => Tile::StoneWall,
            '.' => Tile::OpenPassage,
            '@' => Tile::Entrance,
            _ => (if c.is_lowercase() {
                Tile::Key
            } else {
                Tile::Door
            })(c.to_ascii_uppercase()),
        }
    }

    fn navigable(&self, kr: &Vec<char>) -> bool {
        match self {
            Tile::StoneWall => false,
            Tile::OpenPassage => true,
            Tile::Entrance => true,
            Tile::Key(_) => true,
            Tile::Door(c) => !kr.contains(c),
        }
    }

    fn key(&self, kr: &Vec<char>) -> Option<char> {
        match self {
            Tile::Key(c) => {
                if kr.contains(c) {
                    Some(*c)
                } else {
                    None
                }
            }
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
                Tile::StoneWall => '#',
                Tile::OpenPassage => '.',
                Tile::Entrance => '@',
                Tile::Key(c) => c.to_ascii_lowercase(),
                Tile::Door(c) => *c,
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Vault {
    tiles: Vec<Vec<Tile>>,
}

impl Vault {
    fn iter(&self) -> impl Iterator<Item = ((usize, usize), Tile)> + '_ {
        self.tiles.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, &tile)| ((x, y), tile))
        })
    }
}

impl Index<(usize, usize)> for Vault {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[y][x]
    }
}

impl IndexMut<(usize, usize)> for Vault {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[y][x]
    }
}

impl From<Vec<Vec<Tile>>> for Vault {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        Vault { tiles }
    }
}

impl Display for Vault {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.tiles.iter().map(|l| l.iter().join("")).join("\n")
        )
    }
}

impl Debug for Vault {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn find_shortest(vault: &Vault) -> i32 {
    let mut /* key queue */ kq = PriorityQueue::new();
    let pos = vault
        .iter()
        .filter(|(_, tile)| *tile == Tile::Entrance)
        .map(|(pos, _)| pos)
        .collect_vec();
    let keys_remaining = vault
        .iter()
        .filter_map(|(_, tile)| match tile {
            Tile::Key(c) => Some(c),
            _ => None,
        })
        .collect_vec();
    kq.push((pos, keys_remaining), 0);
    let mut kseen = HashSet::new();

    let mut shortest_path = i32::MAX;
    while !kq.is_empty() {
        let ((positions, keys_remaining), steps) = kq.pop().unwrap();

        if keys_remaining.is_empty() {
            shortest_path = shortest_path.min(-steps);
            continue;
        }

        for (i, &pos) in positions.iter().enumerate() {
            // first find all possible keys, those are the neighbour nodes
            let mut /* intersection */ iq = PriorityQueue::new();
            iq.push(pos, steps);
            let mut iseen = HashSet::new();

            while !iq.is_empty() {
                let (pos, dist) = iq.pop().unwrap();
                match vault[pos].key(&keys_remaining) {
                    Some(k) => {
                        let mut kr = keys_remaining.clone();
                        kr.remove(
                            kr.iter()
                                .position(|x| *x == k)
                                .unwrap_or_else(|| panic!("Duplicate key {}, not in {:?}", k, kr)),
                        );
                        if !kseen.contains(&kr) {
                            let mut poses = positions.clone();
                            poses[i] = pos;
                            let tup = (poses, kr);
                            let dist = (*kq.get_priority(&tup).unwrap_or(&i32::MIN)).max(dist);
                            kq.push(tup, dist);
                        }
                    }
                    None => {
                        // at an intersection, traverse to neighbour intersections
                        for &dir in &DIRECTIONS {
                            let mut mut_dist = dist;
                            let mut mut_pos = dir.offset(pos);
                            while vault[mut_pos].navigable(&keys_remaining) {
                                mut_dist -= 1;
                                if vault[mut_pos].key(&keys_remaining).is_some()
                                    || dir
                                        .turns()
                                        .iter()
                                        .map(|n| vault[n.offset(mut_pos)])
                                        .find(|t| t.navigable(&keys_remaining))
                                        .is_some()
                                {
                                    // another intersection, add to queue, unless seen already
                                    if !iseen.contains(&mut_pos) {
                                        iq.push(
                                            mut_pos,
                                            (*iq.get_priority(&mut_pos).unwrap_or(&i32::MIN))
                                                .max(mut_dist),
                                        );
                                    }
                                    break;
                                }
                                mut_pos = dir.offset(mut_pos);
                            }
                        }
                    }
                }
                iseen.insert(pos);
            }
        }
        kseen.insert(keys_remaining);
    }
    shortest_path
}

#[no_mangle]
pub fn day_18() {
    let stdin = stdin();
    let mut vault: Vault = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().map(Tile::of).collect_vec())
        .collect_vec()
        .into();
    io::println!("Shortest: {}", find_shortest(&vault));

    let pos = vault
        .iter()
        .filter(|(_, tile)| *tile == Tile::Entrance)
        .map(|(pos, _)| pos)
        .next()
        .unwrap();
    vault[pos] = Tile::StoneWall;
    for &dir in DIRECTIONS.iter() {
        let offset = dir.offset(pos);
        vault[offset] = Tile::StoneWall;
        for &turn in &dir.turns() {
            let corner = turn.offset(offset);
            vault[corner] = Tile::Entrance;
        }
    }
    io::println!("Shortest: {}", find_shortest(&vault));
}
