use aoc::intcode::{Program, Int};
use itertools::Itertools;
use aoc::util::Dir;
use std::iter;

const RANGE: Int = 50;
const SHIP_SIZE: Int = 100;

#[allow(unused)]
fn print_map<F: Fn(&(Int, Int)) -> bool>(beam: F, range: Int) {
    println!("{}", (0..range)
        .map(|y| (0..range)
            .map(|x| beam(&(x, y)))
            .map(|b| ['.', '#'][b as usize])
            .join(""))
        .join("\n"));
}

fn main() {
    let beam_raw = Program::from_stdin().unwrap().into_fn();
    let beam = |(x, y): &(Int, Int)| beam_raw(vec![*x, *y])[0] == 1;
    let affected = (0..RANGE).cartesian_product(0..RANGE)
        .filter(beam)
        .count();
    println!("Affected: {}", affected);
    let (x, y) = iter::from_fn({
        let mut last_pos: (Int, Int) = (0, 0);
        move || {
            // note that "north" here is down
            let mut north = Dir::North.offset(last_pos);
            let mut east = Dir::East.offset(last_pos);
            loop {
                if beam(&north) {
                    last_pos = north;
                    break;
                }
                if beam(&east) {
                    last_pos = east;
                    break;
                }
                north = Dir::East.offset(north);
                east = Dir::North.offset(east);
            }
            Some(last_pos)
        }
    })
        .find(|pos| {
            let east = Dir::East.offset_by(*pos, SHIP_SIZE - 1);
            beam(pos) &&
                beam(&east) &&
                beam(&Dir::South.offset_by(*pos, SHIP_SIZE - 1)) &&
                beam(&Dir::South.offset_by(east, SHIP_SIZE - 1))
        })
        .map(|pos| Dir::South.offset_by(pos, SHIP_SIZE - 1))
        .unwrap();
    println!("Coordinates: {} * 10000 + {} = {}", x, y, x * 10000 + y);
}
