use crate::io::{self, stdin, BufRead};
use itertools::Itertools;
use itertools::__std_iter::{FromIterator, Sum};
use num::Integer;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::Add;
use std::str::FromStr;
use std::{fmt, iter};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Vector(i32, i32, i32);

impl Vector {
    fn energy(&self) -> u32 {
        match self {
            Vector(x, y, z) => x.abs() as u32 + y.abs() as u32 + z.abs() as u32,
        }
    }

    fn cmp(&self, Vector(x2, y2, z2): &Self) -> Self {
        match self {
            Vector(x1, y1, z1) => Vector(x1.cmp(x2) as i32, y1.cmp(y2) as i32, z1.cmp(z2) as i32),
        }
    }

    fn axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.0,
            Axis::Y => self.1,
            Axis::Z => self.2,
        }
    }
}

#[derive(Copy, Clone)]
enum Axis {
    X,
    Y,
    Z,
}

fn axes() -> Vec<Axis> {
    vec![Axis::X, Axis::Y, Axis::Z]
}

impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Vector(x, y, z) => write!(f, "<x={}, y={}, z={}>", x, y, z),
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, Vector(x2, y2, z2): Self) -> Self::Output {
        match self {
            Vector(x1, y1, z1) => Vector(x1 + x2, y1 + y2, z1 + z2),
        }
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item = Vector>>(iter: I) -> Self {
        iter.fold(Vector(0, 0, 0), |a, b| a + b)
    }
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.trim_matches(|c| c == '<' || c == '>')
            .split(", ")
            .map(|a| a.split("=").nth(1).unwrap().parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32, i32)>()
            .map(|(x, y, z)| Vector(x, y, z))
            .unwrap())
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Debug for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "pos={:?}, vel={:?}", self.pos, self.vel)
    }
}

impl Moon {
    fn energy(&self) -> u32 {
        self.pos.energy() * self.vel.energy()
    }

    fn with_pos(&self, pos: Vector) -> Self {
        Moon { pos, vel: self.vel }
    }

    fn with_vel(&self, vel: Vector) -> Self {
        Moon { vel, pos: self.pos }
    }
}

impl FromStr for Moon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Moon {
            pos: s.parse::<Vector>().unwrap(),
            vel: Vector(0, 0, 0),
        })
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Universe {
    moons: Vec<Moon>,
}

impl Universe {
    fn simulate(&self) -> impl Iterator<Item = Self> {
        let mut self_mut = self.clone();
        vec![self.clone()].into_iter().chain(iter::from_fn(move || {
            self_mut.moons = self_mut
                .moons
                .iter()
                .map(|m| m.with_vel(m.vel + self_mut.moons.iter().map(|o| o.pos.cmp(&m.pos)).sum()))
                .map(|m| m.with_pos(m.pos + m.vel))
                .collect_vec();
            Some(self_mut.clone())
        }))
    }

    fn axis(&self, axis: Axis) -> Vec<(i32, i32)> {
        self.moons
            .iter()
            .map(|m| (m.pos.axis(axis), m.vel.axis(axis)))
            .collect_vec()
    }

    fn energy(&self) -> u32 {
        self.moons.iter().map(Moon::energy).sum()
    }

    fn period(&self) -> Frequency {
        axes()
            .iter()
            .map(|&axis| self.simulate().map(|u| u.axis(axis)).collect())
            .sum()
    }
}

impl FromIterator<Moon> for Universe {
    fn from_iter<T: IntoIterator<Item = Moon>>(iter: T) -> Self {
        Universe {
            moons: iter.into_iter().collect(),
        }
    }
}

impl Debug for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for moon in &self.moons {
            write!(f, "{:?}\n", moon)?;
        }
        Ok(())
    }
}

// fortunately, it's not necessary to account for phase,
// universes always return to their initial state
struct Frequency(u64);

impl Sum for Frequency {
    fn sum<I: Iterator<Item = Frequency>>(iter: I) -> Self {
        iter.reduce(|a, b| Frequency(a.0.lcm(&b.0)))
            .unwrap_or(Frequency(1))
    }
}

impl Debug for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> FromIterator<T> for Frequency
where
    T: Eq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let mut iter = iterable.into_iter();
        let first: T = iter.next().unwrap();
        Frequency(
            iter.enumerate()
                .find(|(_, state)| *state == first)
                .unwrap()
                .0 as u64
                + 1,
        )
    }
}

#[no_mangle]
pub fn day_12() {
    let stdin = stdin();
    let universe: Universe = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().parse::<Moon>().unwrap())
        .collect();

    io::println!(
        "Energy: {}",
        universe.simulate().nth(1000).unwrap().energy()
    );
    io::println!("Period: {:?}", universe.period());
}


