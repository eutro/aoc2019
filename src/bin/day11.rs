use crate::intcode::{Int, Program, State, VM};
use crate::io;
use num::traits::AsPrimitive;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Copy, Clone)]
enum Colour {
    Black,
    White,
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colour::Black => '.',
                Colour::White => '#',
            }
        )
    }
}

impl AsPrimitive<Int> for Colour {
    fn as_(self) -> Int {
        match self {
            Colour::Black => 0,
            Colour::White => 1,
        }
    }
}

impl Colour {
    fn from(i: Int) -> Self {
        match i {
            0 => Colour::Black,
            _ => Colour::White,
        }
    }
}

fn paint(program: &Program, painted: &mut HashMap<Vector, Colour>) {
    let mut vm = VM::of(program);
    let mut pos = Vector { x: 0, y: 0 };
    let mut look = Vector { x: 0, y: -1 };
    let mut colour = None;
    loop {
        match vm.next_state().unwrap() {
            State::AwaitingInput => {
                vm.input(*painted.get(&pos).unwrap_or(&Colour::Black) as Int);
            }
            State::Outputting(o) => match colour {
                None => {
                    colour = Some(Colour::from(o));
                }
                Some(c) => {
                    painted.insert(pos, c);
                    let turn_right = o != 0;
                    if turn_right {
                        look = Vector {
                            x: -look.y,
                            y: look.x,
                        };
                    } else {
                        look = Vector {
                            x: look.y,
                            y: -look.x,
                        };
                    }
                    pos += look;
                    colour = None;
                }
            },
            State::Finished => break,
        }
    }
}

fn print_hull(painted: &HashMap<Vector, Colour>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (v, _) in painted {
        if v.x < min_x {
            min_x = v.x;
        }
        if v.x > max_x {
            max_x = v.x;
        }
        if v.y < min_y {
            min_y = v.y;
        }
        if v.y > max_y {
            max_y = v.y;
        }
    }
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            let c = painted.get(&Vector { x, y });
            print!("{}", c.unwrap_or(&Colour::Black));
        }
        io::println!();
    }
}

#[no_mangle]
pub fn day_11() {
    let program = Program::from_stdin().unwrap();
    let mut painted;

    painted = HashMap::new();
    paint(&program, &mut painted);
    io::println!("Painted: {}", painted.len());

    painted = HashMap::new();
    painted.insert(Vector { x: 0, y: 0 }, Colour::White);
    paint(&program, &mut painted);
    io::println!("Identifier:");
    print_hull(&painted);
}
