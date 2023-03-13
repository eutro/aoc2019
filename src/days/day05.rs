use crate::intcode::{Int, Program};
use crate::io;

#[no_mangle]
pub fn day_05() {
    let test = Program::from_stdin().unwrap().into_fn();
    let non_zero = test(vec![1])
        .iter()
        .map(|i| *i)
        .filter(|i| *i != 0)
        .collect::<Vec<Int>>();
    if non_zero.len() != 1 {
        panic!("One of the tests failed!")
    }
    io::println!("Code_1: {}", non_zero[0]);
    io::println!("Code_5: {}", test(vec![5])[0]);
}


