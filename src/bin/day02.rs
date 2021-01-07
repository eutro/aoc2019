use aoc::intcode::{Program, VM};

pub fn main() {
    let program = Program::from_stdin().unwrap();
    let mut vm = VM::of(&program);
    vm.mem[1] = 12;
    vm.mem[2] = 2;
    vm.next_state().unwrap();
    println!("Mem_0: {}", vm.mem[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            vm = VM::of(&program);
            vm.mem[1] = noun;
            vm.mem[2] = verb;
            if vm.next_state().is_ok() && vm.mem[0] == 19690720 {
                println!("Sum: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }

    panic!("No combination found")
}
