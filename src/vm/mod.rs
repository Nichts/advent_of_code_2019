pub(crate) mod errors;
mod mode;
mod op;
pub(crate) mod types;

use self::errors::{Error, Result};
use self::mode::Mode;
use self::op::OpCode;
use self::types::Value;

use std::convert::TryInto;

pub trait Memory {
    fn read(&self, address: usize) -> Result<Value>;
    fn write(&mut self, address: usize, value: Value) -> Result<()>;
}

impl Memory for Vec<Value> {
    fn read(&self, address: usize) -> Result<Value> {
        self.get(address).cloned().ok_or(Error::SegFault(address))
    }

    fn write(&mut self, address: usize, value: Value) -> Result<()> {
        self.get_mut(address)
            .map(|val| *val = value)
            .ok_or(Error::SegFault(address))
    }
}

pub struct Computer<M: Memory> {
    memory: M,
    ip: usize,
}

pub enum State {
    Running,
    Halted,
}

fn writing_not_supported(_: Value) -> Result<()> {
    Err(Error::WritingNotSupported)
}

fn reading_not_supported() -> Result<Value> {
    Err(Error::ReadingNotSupported)
}

impl<M: Memory> Computer<M> {
    pub fn new(memory: M) -> Self {
        Self { ip: 0, memory }
    }

    fn step<I, O>(&mut self, read: &mut I, write: &mut O) -> Result<State>
    where
        I: FnMut() -> Result<Value>,
        O: FnMut(Value) -> Result<()>,
    {
        let mut ip = self.ip;
        let mut next_inst = || -> usize {
            let ret = ip;
            ip += 1;
            ret
        };
        let mut inst = self.memory.read(next_inst())?;
        let op_code = (inst % 100).try_into()?;
        inst /= 100;
        let mut pop_mode = || -> Result<Mode> {
            let mode = (inst % 10).try_into();
            inst /= 10;
            mode
        };
        match op_code {
            OpCode::Add => {
                let res =
                    self.read(next_inst(), pop_mode()?)? + self.read(next_inst(), pop_mode()?)?;
                self.write(next_inst(), pop_mode()?, res)?;
            }
            OpCode::Multiply => {
                let res =
                    self.read(next_inst(), pop_mode()?)? * self.read(next_inst(), pop_mode()?)?;
                self.write(next_inst(), pop_mode()?, res)?;
            }
            OpCode::Input => {
                self.write(next_inst(), pop_mode()?, read()?)?;
            }
            OpCode::Output => {
                write(self.read(next_inst(), pop_mode()?)?)?;
            }
            OpCode::Halt => return Ok(State::Halted),
        };
        self.ip = ip;
        Ok(State::Running)
    }

    fn run_all<I, O>(&mut self, read: &mut I, write: &mut O) -> Result<()>
    where
        I: FnMut() -> Result<Value>,
        O: FnMut(Value) -> Result<()>,
    {
        loop {
            match self.step(read, write)? {
                State::Running => (),
                State::Halted => return Ok(()),
            }
        }
    }

    pub fn execute(&mut self) -> Result<Value> {
        self.run_all(&mut reading_not_supported, &mut writing_not_supported)?;
        Ok(self.memory.read(0)?)
    }

    pub fn run<I, O>(&mut self, mut read: I, mut write: O) -> Result<()>
    where
        I: FnMut() -> Result<Value>,
        O: FnMut(Value) -> Result<()>,
    {
        self.run_all(&mut read, &mut write)
    }

    fn read(&self, address: usize, mode: Mode) -> Result<Value> {
        let value = self.memory.read(address);
        match mode {
            Mode::Position => self.memory.read(value? as usize),
            Mode::Immediate => value,
        }
    }

    fn write(&mut self, address: usize, mode: Mode, value: Value) -> Result<()> {
        match mode {
            Mode::Position => self
                .memory
                .write(self.memory.read(address)? as usize, value),
            Mode::Immediate => Err(Error::InvalidWriteMode(mode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut comp = Computer::new(vec![1, 4, 0, 0, 2, 0, 4, 0, 99]);
        assert_eq!(comp.execute().unwrap(), 6)
    }
}
