use crate::intcode::{Program, VM};
use crate::io;

#[no_mangle]
pub fn day_21() {
    let spring_droid = Program::from_stdin().unwrap();
    let mut vm;

    vm = VM::of(&spring_droid);
    vm.input_ascii(
        "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
",
    );
    io::println!("Damage: {}", vm.last().unwrap());

    vm = VM::of(&spring_droid);
    vm.input_ascii(
        "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT H T
NOT T T
OR E T
AND T J
RUN
",
    );
    io::println!("Damage: {}", vm.last().unwrap());
}
