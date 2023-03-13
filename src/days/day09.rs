use crate::intcode::Program;
use crate::io;

#[no_mangle]
pub fn day_09() {
    let boost = Program::from_stdin().unwrap().into_fn();
    io::println!("Keycode: {}", boost(vec![1])[0]);
    io::println!("Coordinates: {}", boost(vec![2])[0]);
}


