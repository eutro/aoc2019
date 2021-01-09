use aoc::intcode::Program;

fn main() {
    let boost = Program::from_stdin().unwrap().into_fn();
    println!("Keycode: {}", boost(vec![1])[0]);
    println!("Coordinates: {}", boost(vec![2])[0]);
}
