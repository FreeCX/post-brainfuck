use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use std::slice;

use crate::command::Command;
use crate::error::{Error, ExecuteError, ParseError};

pub struct Emulator {
    stack: Vec<usize>,
    app: Vec<Command>,
    mem: Vec<u8>,
    ip: usize,
    mp: usize,
}

impl Emulator {
    pub fn new(mem_size: usize) -> Emulator {
        Emulator { stack: Vec::new(), mem: vec![0; mem_size], app: Vec::new(), ip: 0, mp: 0 }
    }

    fn parse_token(&mut self, token: char) -> Result<(), Error> {
        if Command::is_valid_token(token) {
            let index = self.app.len();
            let cmd = match token.into() {
                command @ Command::LoopBegin(_) => {
                    self.stack.push(index);
                    command
                }
                Command::LoopEnd(_) => {
                    let start = match self.stack.pop() {
                        Some(value) => value,
                        None => return Err(ParseError::NoLoopBegin.into()),
                    };
                    self.app[start] = Command::LoopBegin(index);
                    Command::LoopEnd(start)
                }
                cmd => cmd,
            };
            self.app.push(cmd);
        }

        Ok(())
    }

    pub fn from_buffer(&mut self, buffer: &str) -> Result<(), Error> {
        for token in buffer.chars() {
            self.parse_token(token)?;
        }

        match !self.stack.is_empty() {
            true => Err(ParseError::BracketsNumber.into()),
            false => Ok(()),
        }
    }

    pub fn from_stdin(&mut self) -> Result<(), Error> {
        print!("> ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        self.from_buffer(&buffer)
    }

    pub fn from_file<S>(&mut self, filename: &S) -> Result<(), Error>
    where
        S: AsRef<Path> + ?Sized,
    {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        self.from_buffer(&buffer)
    }

    pub fn step(&mut self) -> Result<(), Error> {
        match self.app[self.ip] {
            Command::Next => {
                if self.mp + 1 == self.mem.len() {
                    return Err(ExecuteError::OutOfMemory.into());
                }
                self.mp += 1;
            }
            Command::Previous => {
                if self.mp == 0 {
                    return Err(ExecuteError::NegativeMemoryIndex.into());
                }
                self.mp -= 1;
            }
            Command::Increment => self.mem[self.mp] = self.mem[self.mp].overflowing_add(1).0,
            Command::Decrement => self.mem[self.mp] = self.mem[self.mp].overflowing_sub(1).0,
            Command::Put => {
                print!("{}", self.mem[self.mp] as char);
                io::stdout().flush()?;
            }
            Command::Read => self.mem[self.mp] = io::stdin().bytes().next().ok_or(ExecuteError::ReadStdin)??,
            Command::LoopBegin(index) => {
                if self.mem[self.mp] == 0 {
                    self.ip = index;
                }
            }
            Command::LoopEnd(index) => {
                if self.mem[self.mp] != 0 {
                    self.ip = index;
                }
            }
        }
        self.ip += 1;

        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        while self.ip < self.app.len() {
            self.step()?;
        }

        Ok(())
    }

    pub fn mem_size(&self) -> usize {
        self.mem.len()
    }

    pub fn iter_command(&self) -> slice::Iter<Command> {
        self.app.iter()
    }
}
