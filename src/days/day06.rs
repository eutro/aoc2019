use crate::io::{self, stdin, BufRead};
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

use itertools::Itertools;

fn identify(name: &str) -> u32 {
    name.as_bytes()
        .iter()
        .fold(0, |last, char| last * u8::MAX as u32 + *char as u32)
}

// sum the depths of every node in the tree
fn traverse(obj: u32, orbited: &HashMap<u32, HashSet<u32>>, depth: u32) -> u32 {
    (match orbited.get(&obj) {
        None => 0,
        Some(set) => set.iter().map(|o| traverse(*o, orbited, depth + 1)).sum(),
    }) + depth
}

#[no_mangle]
pub fn day_06() {
    // object -> objects orbiting it
    let mut orbiting_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    // object -> object it's orbiting
    let mut orbited_map: HashMap<u32, u32> = HashMap::new();

    let stdin = stdin();
    for lines in stdin.lock().lines() {
        let (orbited, orbiter) = lines
            .unwrap()
            .split(")")
            .map(identify)
            .collect_tuple()
            .unwrap();
        match orbiting_map.get_mut(&orbited) {
            None => {
                let mut set = HashSet::new();
                set.insert(orbiter);
                orbiting_map.insert(orbited, set);
            }
            Some(set) => {
                set.insert(orbiter);
            }
        }
        orbited_map.insert(orbiter, orbited);
    }

    io::println!("Orbits: {}", traverse(identify("COM"), &orbiting_map, 0));

    let target = *orbited_map.get(&identify("SAN")).unwrap();
    let from = *orbited_map.get(&identify("YOU")).unwrap();

    if target == from {
        io::println!("Jumps: 0");
        return;
    }

    let mut q = VecDeque::new();
    let mut tq = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back(from);
    visited.insert(from);

    let mut jumps = 1;
    while !q.is_empty() {
        for obj in q
            .iter()
            .map(|el| {
                orbiting_map
                    .get(&el)
                    .into_iter()
                    .flatten()
                    .chain(orbited_map.get(&el).into_iter())
            })
            .flatten()
        {
            if *obj == target {
                io::println!("Jumps: {}", jumps);
                return;
            }
            if !visited.insert(*obj) {
                continue;
            };
            tq.push_back(*obj);
        }
        swap(&mut q, &mut tq);
        jumps += 1;
    }
    panic!("No route found!");
}


