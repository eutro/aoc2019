use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn module_fuel(mass: i32) -> i32 { (mass / 3) - 2 }

fn module_fuel_recursive(mass: i32) -> i32 {
    let mut total = 0;
    let mut last_mass = mass;
    loop {
        last_mass = module_fuel(last_mass);
        if last_mass < 0 { break };
        total += last_mass;
    }
    total
}

pub fn run() {
    let f = File::open("input/1.txt").expect("Error reading file");
    let reader = BufReader::new(f);

    let mut fuel = 0;
    let mut fuel_recursive = 0;
    for line in reader.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        fuel += module_fuel(mass);
        fuel_recursive += module_fuel_recursive(mass);
    }
    println!("Fuel: {}", fuel);
    println!("Recursively: {}", fuel_recursive);
}
