use aoc::intcode as ic;

pub fn main() {
    let program = ic::Program::read("input/2.txt");
    let mut vm = ic::VM::of(&program);
    vm.mem[1] = 12;
    vm.mem[2] = 2;
    vm.run().unwrap();
    println!("Mem_0: {}", vm.mem[0]);

    for noun in 0 .. 100 {
        for verb in 0 .. 100 {
            vm = ic::VM::of(&program);
            vm.mem[1] = noun;
            vm.mem[2] = verb;
            if vm.run().is_ok() && vm.mem[0] == 19690720 {
                println!("Sum: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return
            }
        }
    }

    panic!("No combination found")
}
