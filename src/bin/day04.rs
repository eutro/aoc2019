use aoc::numbers::DigitIterable;
use std::io;
use std::io::BufRead;

fn check_p1(pw: &u32) -> bool {
    let mut found_dup = false;
    let mut last_digit = 0;
    for digit in pw.digits() {
        if digit < last_digit {
            return false;
        } else if digit == last_digit {
            found_dup = true;
        }
        last_digit = digit;
    }
    found_dup
}

fn check_p2(pw: &u32) -> bool {
    let mut two_seq = false;
    let mut curr_seq = 1_u8;
    let mut last_digit = 0;
    for digit in pw.digits() {
        if digit < last_digit {
            return false;
        } else if digit == last_digit {
            curr_seq += 1;
        } else {
            if curr_seq == 2 {
                two_seq = true
            }
            curr_seq = 1;
        }
        last_digit = digit;
    }
    two_seq || curr_seq == 2
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let mut numbers = line
        .trim()
        .split("-")
        .map(|s| s.parse::<u32>().unwrap());

    let low = numbers.next().unwrap();
    let hi = numbers.next().unwrap();

    println!("Valid: {}", (low..hi).filter(check_p1).count());
    println!("Valid: {}", (low..hi).filter(check_p2).count())
}
