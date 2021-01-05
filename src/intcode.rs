use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufRead, stdin};
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

use crate::numbers::DigitIterable;

pub type Int = i64;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Int>,
}

#[derive(Debug)]
pub enum ParseProgramError {
    NotInteger(String, ParseIntError),
    IOError(io::Error),
}

impl Program {
    pub fn from_stdin() -> Result<Program, ParseProgramError> {
        let stdin = stdin();
        let mut line = String::new();
        stdin.lock()
            .read_line(&mut line)
            .map_err(|e| ParseProgramError::IOError(e))?;
        line.parse::<Program>()
    }

    pub fn make_fn(self) -> impl Fn(Vec<Int>) -> Vec<Int> {
        move |i| {
            let mut inputs = i.iter();
            let mut out: Vec<Int> = Vec::new();
            VM::of(&self)
                .with_stdin(|| inputs
                    .next()
                    .map(|i| *i)
                    .ok_or(Error::UnsupportedOperation))
                .with_stdout(|i| {
                    out.push(i);
                    Ok(())
                })
                .run()
                .unwrap();
            out
        }
    }
}

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .split(',')
            .map(|s| s
                .parse::<Int>()
                .map_err(|e| ParseProgramError::NotInteger(s.to_owned(), e)))
            .collect::<Result<Vec<Int>, Self::Err>>()?;
        Ok(Program { instructions })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
    UnrecognisedMode(u8),
    UnrecognisedOpcode(u8),
    UnsupportedOperation,
    MemoryOutOfBounds(Int),
    UnterminatedProgram(usize),
    UnsupportedSet(Mode),
}

pub struct ExecError {
    mem: Vec<Int>,
    insn: usize,
    error: Error,
}

type VMResult<T> = Result<T, Error>;
type ExecResult<T> = Result<T, ExecError>;

#[derive(Copy, Clone, Debug)]
pub enum Insn {
    Add,
    Mul,
    Input,
    Output,
    End,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

impl Insn {
    pub fn of(opcode: u8) -> VMResult<Self> {
        Ok(match opcode {
            1 => Insn::Add,
            2 => Insn::Mul,
            3 => Insn::Input,
            4 => Insn::Output,
            5 => Insn::JumpIfTrue,
            6 => Insn::JumpIfFalse,
            7 => Insn::LessThan,
            8 => Insn::Equals,

            99 => Insn::End,

            i => return Err(Error::UnrecognisedOpcode(i))
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn of(digit: u8) -> VMResult<Self> {
        Ok(match digit {
            0 => Mode::Position,
            1 => Mode::Immediate,

            d => return Err(Error::UnrecognisedMode(d))
        })
    }
}

pub struct VM<I: FnMut() -> VMResult<Int>, O: FnMut(Int) -> VMResult<()>> {
    pub mem: Vec<Int>,
    pub insn: usize,
    stdin: I,
    stdout: O,
}

impl VM<fn() -> VMResult<Int>, fn(Int) -> VMResult<()>> {
    pub fn of(program: &Program) -> Self {
        VM {
            mem: program.instructions.clone(),
            insn: 0,
            stdin: || {
                Err(Error::UnsupportedOperation)
            },
            stdout: |i| {
                print!("{}", i);
                Ok(())
            },
        }
    }
}

impl<I: FnMut() -> VMResult<Int>, O: FnMut(Int) -> VMResult<()>> VM<I, O> {
    pub fn with_stdin<NewI: FnMut() -> VMResult<i64>>(self, stdin: NewI) -> VM<NewI, O> {
        VM {
            mem: self.mem,
            insn: self.insn,
            stdin,
            stdout: self.stdout,
        }
    }

    pub fn with_stdout<NewO: FnMut(Int) -> VMResult<()>>(self, stdout: NewO) -> VM<I, NewO> {
        VM {
            mem: self.mem,
            insn: self.insn,
            stdin: self.stdin,
            stdout,
        }
    }

    fn peek(&self) -> VMResult<Int> {
        self.mem
            .get(self.insn)
            .map(|i| *i)
            .ok_or(Error::UnterminatedProgram(self.insn))
    }

    fn poll(&mut self) -> VMResult<Int> {
        let ret = self.peek();
        self.insn += 1;
        ret
    }

    fn maybe_resize(&mut self, idx: usize) {
        if idx as usize >= self.mem.len() {
            self.mem.resize(idx as usize + 1, 0);
        }
    }

    fn get<Iter: Iterator<Item=VMResult<Mode>>>(&mut self, modes: &mut Iter) -> VMResult<Int> {
        let v = self.poll()?;
        Ok(match modes.next().unwrap()? {
            Mode::Position => {
                if v < 0 || v as usize >= self.mem.len() {
                    return Err(Error::MemoryOutOfBounds(v));
                }
                self.maybe_resize(v as usize);
                self.mem[v as usize]
            }
            Mode::Immediate => v,
        })
    }

    fn set<Iter: Iterator<Item=VMResult<Mode>>>(&mut self, modes: &mut Iter, val: Int) -> VMResult<()> {
        let idx = self.poll()?;
        Ok(match modes.next().unwrap()? {
            Mode::Position => {
                if idx < 0 || idx as usize >= usize::MAX {
                    return Err(Error::MemoryOutOfBounds(idx));
                } else {
                    self.maybe_resize(idx as usize);
                    self.mem[idx as usize] = val;
                }
            }
            mode => return Err(Error::UnsupportedSet(mode))
        })
    }

    fn jump(&mut self, to: Int) -> VMResult<()> {
        if to < 0 || to as usize >= usize::MAX {
            Err(Error::MemoryOutOfBounds(to))
        } else {
            self.insn = to as usize;
            Ok(())
        }
    }

    fn advance(&mut self) -> VMResult<bool> {
        let modes =
            &mut ((self.peek()? / 100) as u32)
                .reverse_digits()
                .map(Mode::of);
        match Insn::of((self.poll()? % 100) as u8)? {
            Insn::Add => {
                let sum = self.get(modes)? + self.get(modes)?;
                self.set(modes, sum)?
            }
            Insn::Mul => {
                let prod = self.get(modes)? * self.get(modes)?;
                self.set(modes, prod)?
            }
            Insn::Input => {
                let i = (self.stdin)()?;
                self.set(modes, i)?
            }
            Insn::Output => {
                let i = self.get(modes)?;
                (self.stdout)(i)?
            }

            Insn::End => return Ok(false),
            Insn::JumpIfTrue => {
                let pred = self.get(modes)?;
                let to = self.get(modes)?;
                if pred != 0 {
                    self.jump(to)?;
                }
            }
            Insn::JumpIfFalse => {
                let pred = self.get(modes)?;
                let to = self.get(modes)?;
                if pred == 0 {
                    self.jump(to)?;
                }
            }
            Insn::LessThan => {
                let val = if self.get(modes)? < self.get(modes)? {
                    1
                } else {
                    0
                };
                self.set(modes, val)?;
            }
            Insn::Equals => {
                let val = if self.get(modes)? == self.get(modes)? {
                    1
                } else {
                    0
                };
                self.set(modes, val)?;
            }
        }
        Ok(true)
    }

    pub fn run(&mut self) -> ExecResult<()> {
        while self.advance()
            .map_err(|error| ExecError {
                mem: self.mem.clone(),
                error,
                insn: self.insn - 1,
            })? {}
        Ok(())
    }
}

fn format_mem(f: &mut Formatter, mem: &Vec<Int>, insn: usize) -> fmt::Result {
    mem.iter()
        .take(insn)
        .map(|v| write!(f, "{},", v))
        .collect::<fmt::Result>()?;
    let mut iter = mem
        .iter()
        .skip(insn);
    write!(f, "[")?;
    iter.next()
        .map(|val| write!(f, "{}],", val))
        .unwrap_or(Ok(()))?;
    iter.map(|v| write!(f, "{},", v))
        .collect::<fmt::Result>()?;
    write!(f, "END")?;
    Ok(())
}

impl<I: FnMut() -> VMResult<Int>, O: FnMut(Int) -> VMResult<()>> Debug for VM<I, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format_mem(f, &self.mem, self.insn)
    }
}

impl Debug for ExecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\n", self.error)?;
        format_mem(f, &self.mem, self.insn)
    }
}
