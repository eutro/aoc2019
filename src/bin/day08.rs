use std::io::{BufRead, stdin};

use itertools::{Itertools};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn count_digit(digit: u32, layer: &Vec<u32>) -> usize {
    layer.iter()
        .filter(|i| **i == digit)
        .count()
}

fn main() {
    let stdin = stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let layers: Vec<Vec<u32>> = line
        .trim()
        .chars()
        .map(|c| c
            .to_digit(10)
            .unwrap())
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .map(|layer| layer.collect_vec())
        .collect_vec();

    let min_layer = layers
        .iter()
        .min_by_key(|layer| count_digit(0, layer))
        .unwrap();

    let ones = count_digit(1, min_layer);
    let twos = count_digit(2, min_layer);
    println!("Product: {} * {} = {}", ones, twos, ones * twos);

    println!("Image:");
    for line in layers
        .into_iter()
        .map(|v| v.into_iter())
        .fold1(|above, below| above
            .zip(below)
            .map(|(ap, bp)| if ap == 2 { bp } else { ap })
            .collect_vec()
            .into_iter())
        .unwrap()
        .chunks(WIDTH)
        .into_iter()
    {
        for pix in line {
            print!("{}", if pix == 0 { ' ' } else { '#' });
        }
        println!();
    }
}
