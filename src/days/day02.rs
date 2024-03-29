use crate::intcode::{Program, VM};
use crate::io;

#[no_mangle]
pub fn day_02() {
    let program = Program::from_stdin().unwrap();
    let mut vm = VM::of(&program);
    vm.mem[1] = 12;
    vm.mem[2] = 2;
    vm.next_state().unwrap();
    io::println!("Mem_0: {}", vm.mem[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            vm = VM::of(&program);
            vm.mem[1] = noun;
            vm.mem[2] = verb;
            if vm.next_state().is_ok() && vm.mem[0] == 19690720 {
                io::println!("Sum: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }

    panic!("No combination found")
}


