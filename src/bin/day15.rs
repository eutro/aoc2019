use crate::intcode::{Int, Program, State, VM};
use crate::io;
use crate::util::DIRECTIONS;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::swap;

enum Tile {
    Wall = 0,
    Space = 1,
    Oxygen = 2,
}

#[allow(unused)]
fn print_positions(positions: &HashMap<(i32, i32), Tile>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (&(x, y), _) in positions {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            print!(
                "{}",
                match positions.get(&(x, y)) {
                    None => ' ',
                    Some(t) => match t {
                        Tile::Wall => '#',
                        Tile::Space => '.',
                        Tile::Oxygen => 'O',
                    },
                }
            )
        }
        io::println!();
    }
    io::println!()
}

#[no_mangle]
pub fn day_15() {
    let program = Program::from_stdin().unwrap();

    let mut droids = VecDeque::new();
    let mut tq = VecDeque::new();
    let mut seen = HashSet::new();
    droids.push_back(((0, 0), VM::of(&program)));
    seen.insert((0, 0));
    let mut steps = 0;

    let mut positions = HashMap::new();
    let mut oxygen_source = (0, 0);

    while !droids.is_empty() {
        steps += 1;
        for (pos, vm) in &droids {
            for &dir in &DIRECTIONS {
                let new_pos = dir.offset(*pos);
                if seen.insert(new_pos) {
                    let mut new_vm = vm.clone();
                    new_vm.input(dir as Int);
                    match new_vm.next_state().unwrap() {
                        State::Outputting(status) => {
                            positions.insert(
                                new_pos,
                                match status {
                                    0 => Tile::Wall,
                                    1 => {
                                        tq.push_back((new_pos, new_vm));
                                        Tile::Space
                                    }
                                    2 => {
                                        io::println!("Steps: {}", steps);
                                        oxygen_source = new_pos;
                                        Tile::Oxygen
                                    }
                                    _ => panic!(),
                                },
                            );
                        }
                        _ => panic!(),
                    }
                }
            }
        }
        droids.clear();
        swap(&mut droids, &mut tq);
    }
    tq.clear();

    let mut oxq = VecDeque::new();
    let mut oxqt = VecDeque::new();
    let mut minutes = 0;
    oxq.push_back(oxygen_source);

    loop {
        for &pos in &oxq {
            for &dir in &DIRECTIONS {
                let new_pos = dir.offset(pos);
                match positions.get(&new_pos) {
                    Some(Tile::Space) => {
                        positions.insert(new_pos, Tile::Oxygen);
                        oxqt.push_back(new_pos);
                    }
                    _ => {}
                }
            }
        }
        oxq.clear();
        swap(&mut oxq, &mut oxqt);
        if oxq.is_empty() {
            break;
        }
        minutes += 1;
    }
    io::println!("Minutes: {}", minutes);
}
