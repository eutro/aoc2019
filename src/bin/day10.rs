use std::collections::HashSet;
use std::io::{BufRead, stdin};

use itertools::Itertools;
use num::Integer;
use std::f32::consts::PI;

fn get_detectable(from: &(i32, i32), asteroids: &HashSet<(i32, i32)>, width: i32, height: i32) -> HashSet<(i32, i32)> {
    let mut detectable = HashSet::new();
    let mut candidates = asteroids.clone();
    let (sx, sy) = from;
    candidates.remove(&(*sx, *sy));
    loop {
        match candidates.iter().next() {
            None => break,
            Some(/* asteroid */ (ax, ay)) => {
                let /* relative */ (rx, ry) = (ax - sx, ay - sy);
                let gcd = rx.gcd(&ry);
                let /* lowest */ (lx, ly) = (rx / gcd, ry / gcd);
                let /* current */ (mut cx, mut cy) = (*sx, *sy);
                let mut found = false;
                loop {
                    cx += lx;
                    cy += ly;
                    if cx < 0 || width <= cx ||
                        cy < 0 || height <= cy
                    {
                        break;
                    }
                    if candidates.remove(&(cx, cy)) && !found {
                        detectable.insert((cx, cy));
                        found = true;
                    }
                }
            }
        }
    }
    return detectable;
}

// where north is negative y, and east is positive x
fn bearing((x, y): (i32, i32)) -> f32 {
    let res = (x as f32).atan2(-y as f32);
    if res < 0_f32 {
        2_f32 * PI + res
    } else {
        res
    }
}

const ASTEROID_NTH: usize = 200;

fn main() {
    let stdin = stdin();
    let mut asteroids: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);
    for line in stdin.lock().lines() {
        x = 0;
        for c in line.unwrap().chars() {
            if c == '#' {
                asteroids.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    let (width, height) = (x, y);

    let (pos, mut detectable) =
        asteroids.iter()
            .map(|station| {
                (*station, get_detectable(station, &asteroids, width, height))
            })
            .max_by_key(|(_, visible)| visible.len())
            .unwrap();
    let (sx, sy) = pos;

    println!("Visible: {} from {:?}", detectable.len(), pos);

    asteroids.remove(&pos);
    let mut vaporized = 0;
    while !asteroids.is_empty() {
        if vaporized + detectable.len() >= ASTEROID_NTH {
            let (tx, ty) = detectable
                .into_iter()
                .sorted_by_key(|(ax, ay)| {
                    let rel = (ax - sx, ay - sy);
                    let angle = bearing(rel);
                    // Rust is based and I can't sort by floats directly
                    (angle * 1000_f32) as i32
                })
                .nth(ASTEROID_NTH - vaporized - 1)
                .unwrap();
            println!("200th: {} * 100 + {} = {}",
                     tx, ty,
                     tx * 100 + ty);
            return;
        } else {
            vaporized += detectable.len();
            for asteroid in dbg!(detectable) {
                asteroids.remove(&asteroid);
            }
        }
        detectable = get_detectable(&pos, &asteroids, width, height);
    }
    panic!("Didn't find 200th")
}
