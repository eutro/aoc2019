use std::io::{BufRead, stdin, stdout, Write};

use aoc::intcode::{Program, VM, Int, State};
use std::env;

fn main() {
    let ascii: bool = env::var("INTCODE_ASCII")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap();
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
            match vm.next_state() {
                Err(e) => {
                    eprintln!("Error running Intcode program: {:?}", e);
                    break;
                }
                Ok(State::AwaitingInput) => {
                    if !ascii {
                        print!("> ");
                        stdout().flush().unwrap();
                    }
                    let stdin = stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).unwrap();
                    if ascii {
                        vm.input_ascii(&*line.into_boxed_str());
                    } else {
                        match line.trim().parse::<Int>() {
                            Ok(v) => vm.input(v),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                }
                Ok(State::Outputting(i)) => {
                    printed = true;
                    if ascii {
                        print!("{}", i as u8 as char);
                    } else {
                        println!("{}", i);
                    }
                }
                Ok(State::Finished) => {
                    if !printed {
                        println!("{:?}", vm.mem);
                    }
                    break;
                }
            }
        }
    }
}
