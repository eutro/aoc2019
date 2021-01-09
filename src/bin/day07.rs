use itertools::Itertools;

use aoc::intcode::{Program, VM, State};

const ACCELERATOR_COUNT: usize = 5;

fn main() {
    let program = Program::from_stdin().unwrap();
    let acs = program.clone().into_fn();
    println!("Max: {}", vec![0, 1, 2, 3, 4]
        .iter()
        .permutations(ACCELERATOR_COUNT)
        .map(|seq| seq
            .iter()
            .fold(0, |out, sig| acs(vec![**sig, out])[0]))
        .max()
        .unwrap());

    println!("Max: {}", vec![5, 6, 7, 8, 9]
        .iter()
        .permutations(ACCELERATOR_COUNT)
        .map(|seq| {
            let mut vms = seq.iter()
                .map(|sig| {
                    let mut vm = VM::of(&program);
                    vm.input(**sig);
                    vm
                })
                .collect_vec();
            vms[0].input(0);
            loop {
                let mut deadlock_check = true;
                for accelerator in 0..ACCELERATOR_COUNT {
                    loop {
                        let state = vms.get_mut(accelerator).unwrap().next_state().unwrap();
                        match state {
                            State::Outputting(i) => {
                                let next_vm = vms.get_mut((accelerator + 1) % ACCELERATOR_COUNT).unwrap();
                                if next_vm.is_finished() {
                                    return i;
                                }
                                next_vm.input(i);
                                deadlock_check = false;
                            },
                            State::AwaitingInput => break,
                            State::Finished => break,
                        }
                    }
                }
                if deadlock_check {
                    // take that, halting problem
                    panic!("Deadlock detected!");
                }
            }
        })
        .max()
        .unwrap());
}
