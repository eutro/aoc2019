use crate::io::{self, stdin, BufRead};
use std::collections::VecDeque;
use std::fmt::{self, Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

use crate::numbers::DigitIterable;

pub type Int = i64;

#[derive(Debug, Clone)]
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
        stdin
            .lock()
            .read_line(&mut line)
            .map_err(|e| ParseProgramError::IOError(e))?;
        line.parse::<Program>()
    }

    pub fn into_fn(self) -> impl Fn(Vec<Int>) -> Vec<Int> {
        move |input| {
            let mut vm = VM::of(&self);
            for i in input {
                vm.input(i);
            }
            let mut ret = Vec::new();
            loop {
                match vm.next_state() {
                    Ok(State::Outputting(i)) => ret.push(i),
                    Ok(State::Finished) => return ret,
                    Ok(s) => panic!("Unexpected state: {:?}", s),
                    Err(e) => panic!("{:?}", e),
                }
            }
        }
    }
}

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .split(',')
            .map(|s| {
                s.parse::<Int>()
                    .map_err(|e| ParseProgramError::NotInteger(s.to_owned(), e))
            })
            .collect::<Result<Vec<Int>, Self::Err>>()?;
        Ok(Program { instructions })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum State {
    AwaitingInput,
    Outputting(Int),
    Finished,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    SetBase,
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
            9 => Insn::SetBase,

            99 => Insn::End,

            i => return Err(Error::UnrecognisedOpcode(i)),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    pub fn of(digit: u8) -> VMResult<Self> {
        Ok(match digit {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,

            d => return Err(Error::UnrecognisedMode(d)),
        })
    }
}

#[derive(Clone)]
pub struct VM {
    pub mem: Vec<Int>,
    insn: usize,
    inbuf: VecDeque<Int>,
    relbase: Int,
}

impl VM {
    pub fn of(program: &Program) -> Self {
        VM {
            mem: program.instructions.clone(),
            insn: 0,
            inbuf: VecDeque::new(),
            relbase: 0,
        }
    }

    pub fn next_state(&mut self) -> ExecResult<State> {
        loop {
            match self.advance().map_err(|error| ExecError {
                mem: self.mem.clone(),
                error,
                insn: self.insn - 1,
            })? {
                None => (),
                Some(state) => return Ok(state),
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.peek()
            .map(|o| Insn::of((o % 100) as u8).unwrap_or(Insn::End))
            .unwrap_or(Insn::End)
            == Insn::End
    }

    pub fn input(&mut self, input: Int) {
        self.inbuf.push_back(input);
    }

    pub fn input_ascii(&mut self, input: &str) {
        for c in input.chars() {
            self.input(c as Int);
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

    fn get<Iter: Iterator<Item = VMResult<Mode>>>(&mut self, modes: &mut Iter) -> VMResult<Int> {
        let v = self.poll()?;
        Ok(match modes.next().unwrap()? {
            Mode::Position => {
                if v < 0 {
                    return Err(Error::MemoryOutOfBounds(v));
                }
                self.maybe_resize(v as usize);
                self.mem[v as usize]
            }
            Mode::Immediate => v,
            Mode::Relative => {
                let t = self.relbase + v;
                if t < 0 {
                    return Err(Error::MemoryOutOfBounds(v));
                }
                self.maybe_resize(t as usize);
                self.mem[t as usize]
            }
        })
    }

    fn set<Iter: Iterator<Item = VMResult<Mode>>>(
        &mut self,
        modes: &mut Iter,
        val: Int,
    ) -> VMResult<()> {
        let v = self.poll()?;
        Ok(match modes.next().unwrap()? {
            Mode::Position => {
                if v < 0 {
                    return Err(Error::MemoryOutOfBounds(v));
                }
                self.maybe_resize(v as usize);
                self.mem[v as usize] = val;
            }
            Mode::Relative => {
                let t = self.relbase + v;
                if t < 0 {
                    return Err(Error::MemoryOutOfBounds(v));
                }
                self.maybe_resize(t as usize);
                self.mem[t as usize] = val;
            }
            mode => return Err(Error::UnsupportedSet(mode)),
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

    fn advance(&mut self) -> VMResult<Option<State>> {
        let modes = &mut ((self.peek()? / 100) as u32).reverse_digits().map(Mode::of);
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
                let input = self.inbuf.pop_front();
                if input.is_some() {
                    self.set(modes, input.unwrap())?
                } else {
                    self.insn -= 1;
                    return Ok(Some(State::AwaitingInput));
                }
            }
            Insn::Output => {
                let i = self.get(modes)?;
                return Ok(Some(State::Outputting(i)));
            }
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
            Insn::SetBase => {
                let adj = self.get(modes)?;
                self.relbase += adj;
            }

            Insn::End => {
                self.insn -= 1; // keep the program terminated
                return Ok(Some(State::Finished));
            }
        }
        Ok(None)
    }
}

fn format_mem(f: &mut Formatter, mem: &Vec<Int>, insn: usize) -> fmt::Result {
    mem.iter()
        .take(insn)
        .map(|v| write!(f, "{},", v))
        .collect::<fmt::Result>()?;
    let mut iter = mem.iter().skip(insn);
    write!(f, "[")?;
    iter.next()
        .map(|val| write!(f, "{}],", val))
        .unwrap_or(Ok(()))?;
    iter.map(|v| write!(f, "{},", v)).collect::<fmt::Result>()?;
    write!(f, "END")?;
    Ok(())
}

impl Debug for VM {
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

impl Iterator for VM {
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        match self.next_state() {
            Ok(s) => match s {
                State::AwaitingInput => None,
                State::Outputting(i) => Some(i),
                State::Finished => None,
            },
            Err(_) => None,
        }
    }
}
