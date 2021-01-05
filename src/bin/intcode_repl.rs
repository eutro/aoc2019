use std::io::{BufRead, stdin, stdout, Write};

use aoc::intcode::{Program, VM, Int};

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
        let mut vm = VM::of(&program.unwrap())
            .with_stdout(|i| {
                print!("{}", i);
                printed = true;
                Ok(())
            })
            .with_stdin(|| {
                print!("> ");
                stdout().flush().unwrap();
                let mut line = String::new();
                stdin().lock().read_line(&mut line).unwrap();
                Ok(line.trim().parse::<Int>().unwrap())
            });
        let result = vm.run();
        if result.is_err() {
            eprintln!("Error running Intcode program: {:?}", result.unwrap_err());
        } else {
            let mem = vm.mem.clone();
            if printed {
                println!();
            } else {
                println!("{:?}", mem);
            }
        }
    }
}
