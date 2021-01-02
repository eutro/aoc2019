use std::vec::Vec;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<i32> 
}

#[derive(Debug)]
pub struct VM {
    pub mem: Vec<i32>,
    insn: i32
}

#[derive(PartialEq, Eq)]
pub enum State {
    Advancing,
    Finished,
}

#[derive(Debug)]
pub struct Error {
    cause: Cause
}

#[derive(Debug)]
enum Cause {
    Unrecognised(i32)
}

impl Program {
    pub fn read<P: AsRef<Path>>(path: P) -> Program {
        Program {
            instructions:
            fs::read_to_string(path).expect("Error reading file")
                .trim()
                .split(',')
                .map(|s| s.parse::<i32>()
                     .unwrap_or_else(|e| panic!("Intcode program must be made up of, well, ints, not \"{}\": {}", s, e)))
                .collect()
        }
    }
}

impl VM {
    pub fn of(program: &Program) -> VM {
        VM {
            mem: program.instructions.clone(),
            insn: 0
        }
    }

    fn absref(&self, addr: i32) -> i32 { self.mem[addr as usize] }
    fn absset(&mut self, addr: i32, val: i32) { self.mem[addr as usize] = val }
    fn relref(&self, addr: i32) -> i32 { self.mem[(self.insn + addr) as usize] }
    fn advprog(&mut self, addr: i32) { self.insn += addr }

    pub fn advance(&mut self) -> Result<State, Error> {
        match self.relref(0) {
            1 => {
                self.absset(self.relref(3), self.absref(self.relref(1)) + self.absref(self.relref(2)));
                self.advprog(4);
                Ok(State::Advancing)
            }
            2 => {
                self.absset(self.relref(3), self.absref(self.relref(1)) * self.absref(self.relref(2)));
                self.advprog(4);
                Ok(State::Advancing)
            },
            99 => Ok(State::Finished),
            op => Err(Error { cause: Cause::Unrecognised(op) })
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        while self.advance()? != State::Finished { };
        Ok(())
    }
}
