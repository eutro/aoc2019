use std::io::{BufRead, stdin, stdout, Write};

use aoc::intcode::{Program, VM, Int, State};

fn main() {
    loop {
        print!(">>> ");
        stdout().flush().unwrap();
        let program = Program::from_stdin();
        if !program.is_ok() {
            eprintln!("Error reading Intcode program: {:?}", program.unwrap_err());
            continue;
        }
        let mut printed = false;
        let mut vm = VM::of(&program.unwrap());
        loop {
            match vm.run() {
                Err(e) => {
                    eprintln!("Error running Intcode program: {:?}", e);
                    break;
                }
                Ok(State::AwaitingInput) => {
                    print!("> ");
                    stdout().flush().unwrap();
                    let stdin = stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).unwrap();
                    match line.trim().parse::<Int>() {
                        Ok(v) => vm.input(v),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Ok(State::Outputting(i)) => {
                    printed = true;
                    println!("{}", i);
                }
                Ok(State::Finished) => {
                    if !printed {
                        println!("{:?}", vm.mem);
                    }
                    break;
                }
                Ok(_) => (),
            }
        }
    }
}
