mod days;
mod intcode;

const DAY_COUNT: u32 = 2;
const DAYS: [fn(); DAY_COUNT as usize] = [
    days::day1::run, days::day2::run
];

fn run_day(day: u32) {
    if day > DAY_COUNT {
        panic!("Day out of bounds: {}", day)
    }
    println!("Day {}:", day);
    DAYS[(day - 1) as usize]();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        for day in 1 .. DAY_COUNT + 1 {
            run_day(day);
        }
    } else {
        for day_i in 1 .. args.len() {
            run_day(args[day_i].parse::<u32>().unwrap());
        }
    }
}
