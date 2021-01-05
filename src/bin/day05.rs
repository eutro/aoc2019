use aoc::intcode::{Int, Program};

fn main() {
    let program = Program::from_stdin().unwrap();
    let test = program.as_fn();
    let non_zero = test(vec![1])
        .iter()
        .map(|i| *i)
        .filter(|i| *i != 0)
        .collect::<Vec<Int>>();
    if non_zero.len() != 1 {
        panic!("One of the tests failed!")
    }
    println!("Code_1: {}", non_zero[0]);
    println!("Code_5: {}", test(vec![5])[0]);
}
