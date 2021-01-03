use std::cmp::max;
use std::cmp::min;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct P(i32, i32);

fn to_points<I: Iterator<Item = String>>(points: I) -> Vec<P> {
    let mut last: P = P(0, 0);
    let mut ret = Vec::new();
    for spec in points {
        let mut chars = spec.chars();
        let dir = chars.next().unwrap();
        let dist_s = chars.as_str();
        let dist = dist_s
            .parse::<i32>()
            .unwrap_or_else(|e| panic!("Parse error on {}: {}", dist_s, e));
        let P(lx, ly) = last;
        last = match dir {
            'R' => P(lx + dist, ly),
            'L' => P(lx - dist, ly),
            'U' => P(lx, ly + dist),
            'D' => P(lx, ly - dist),
            c => panic!("Unexpected char: {}", c),
        };
        ret.push(last);
    }
    ret
}

fn read_wire() -> Vec<P> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    to_points(line.trim().split(",").map(String::from))
}

#[derive(PartialEq, Eq)]
enum Orientation {
    Colinear,
    Clockwise,
    Counterclockwise,
}

fn orientation(p: &P, q: &P, r: &P) -> Orientation {
    let val = ((q.1 - p.1) * (r.0 - q.0)) - ((q.0 - p.0) * (r.1 - q.1));

    if val == 0 {
        Orientation::Colinear
    } else if val > 0 {
        Orientation::Clockwise
    } else {
        Orientation::Counterclockwise
    }
}

fn on_segment(p: &P, q: &P, r: &P) -> bool {
    q.0 <= max(p.0, r.0) && q.0 >= min(p.0, r.0) && q.1 <= max(p.1, r.1) && q.1 >= min(p.1, r.1)
}

fn intersect(a1: &P, a2: &P, b1: &P, b2: &P) -> Option<P> {
    let o1 = orientation(a1, a2, b1);
    let o2 = orientation(a1, a2, b2);
    let o3 = orientation(b1, b2, a1);
    let o4 = orientation(b1, b2, a2);

    if o1 != o2 && o3 != o4
        || (o1 == Orientation::Colinear && on_segment(a1, b1, a2))
        || (o2 == Orientation::Colinear && on_segment(a1, b2, a2))
        || (o3 == Orientation::Colinear && on_segment(b1, a1, b2))
        || (o4 == Orientation::Colinear && on_segment(b1, a2, b2))
    {
        if a1.0 == a2.0 {
            Some(P(a1.0, b1.1))
        } else {
            Some(P(b1.0, a1.1))
        }
    } else {
        None
    }
}

fn seg_len(p: &P, q: &P) -> i32 {
    if p.0 == q.0 {
        (p.1 - q.1).abs()
    } else {
        (p.0 - q.0).abs()
    }
}

pub fn main() {
    let wire_a = read_wire();
    let wire_b = read_wire();

    let mut ci_manhattan = i32::MAX;
    let mut ci_steps = i32::MAX;

    let mut last_a = P(0, 0);
    let mut step_a = 0;
    for point_a in &wire_a {
        let mut last_b = P(0, 0);
        let mut step_b = 0;
        for point_b in &wire_b {
            let intersection = intersect(&last_a, point_a, &last_b, point_b);
            if intersection.is_some() {
                let c = intersection.unwrap();
                if c.0 != 0 || c.1 != 0 {
                    let manhattan = c.0.abs() + c.1.abs();
                    if manhattan < ci_manhattan {
                        ci_manhattan = manhattan;
                    }
                    let steps = step_a + step_b + seg_len(&last_a, &c) + seg_len(&last_b, &c);
                    if steps < ci_steps {
                        ci_steps = steps;
                    }
                }
            }

            step_b += seg_len(&last_b, point_b);
            last_b = *point_b;
        }
        step_a += seg_len(&last_a, point_a);
        last_a = *point_a;
    }

    println!("Manhattan: {}", ci_manhattan);
    println!("Steps: {}", ci_steps);
}
