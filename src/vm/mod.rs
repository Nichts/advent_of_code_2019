pub(crate) mod errors;
mod mode;
mod op;
pub(crate) mod types;

use self::errors::{Error, Result};
use self::mode::Mode;
use self::op::OpCode;
use self::types::Value;

use std::cmp::Ordering;
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
            OpCode::JumpIfTrue => {
                if let Some(new_ip) = self.jump_if(true, &mut next_inst, &mut pop_mode)? {
                    ip = new_ip
                }
            }
            OpCode::JumpIfFalse => {
                if let Some(new_ip) = self.jump_if(false, &mut next_inst, &mut pop_mode)? {
                    ip = new_ip
                }
            }
            OpCode::LessThan => {
                self.write_if(Ordering::Less, &mut next_inst, &mut pop_mode)?;
            }
            OpCode::Equals => {
                self.write_if(Ordering::Equal, &mut next_inst, &mut pop_mode)?;
            }
            OpCode::Halt => return Ok(State::Halted),
        };
        self.ip = ip;
        Ok(State::Running)
    }

    fn jump_if(
        &mut self,
        nonzero: bool,
        next_inst: &mut dyn FnMut() -> usize,
        pop_mode: &mut dyn FnMut() -> Result<Mode>,
    ) -> Result<Option<usize>> {
        let zero = self.read(next_inst(), pop_mode()?)?.eq(&0);
        let target = self.read(next_inst(), pop_mode()?)?;
        if zero ^ nonzero {
            Ok(Some(target as usize))
        } else {
            Ok(None)
        }
    }

    fn write_if(
        &mut self,
        order: Ordering,
        next_inst: &mut dyn FnMut() -> usize,
        pop_mode: &mut dyn FnMut() -> Result<Mode>,
    ) -> Result<()> {
        let res = self
            .read(next_inst(), pop_mode()?)?
            .cmp(&self.read(next_inst(), pop_mode()?)?);
        let value = if res == order { 1 } else { 0 };
        self.write(next_inst(), pop_mode()?, value)?;
        Ok(())
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

    fn create_inst() -> Box<dyn FnMut() -> usize> {
        let mut ip = 0;
        let next_inst = move || -> usize {
            let ret = ip;
            ip += 1;
            ret
        };
        Box::new(next_inst)
    }

    #[test]
    fn test_jump_if() -> Result<()> {
        let mut comp = Computer::new(vec![/*5 | 6 */ 1, 5]);
        let mut pop_mode = || -> Result<Mode> { Ok(Mode::Immediate) };
        assert_eq!(
            comp.jump_if(true, &mut create_inst(), &mut pop_mode)?,
            Some(5)
        );
        assert_eq!(
            comp.jump_if(false, &mut create_inst(), &mut pop_mode)?,
            None
        );
        Ok(())
    }

    #[test]
    fn write_if() -> Result<()> {
        let mut comp = Computer::new(vec![/*7 | 8 */ 3, 4, 5, 1, 2, -1]);
        let mut pop_mode = || -> Result<Mode> { Ok(Mode::Position) };
        comp.write_if(Ordering::Less, &mut create_inst(), &mut pop_mode)?;
        assert_eq!(comp.read(5, Mode::Immediate)?, 1);
        comp.write_if(Ordering::Equal, &mut create_inst(), &mut pop_mode)?;
        assert_eq!(comp.read(5, Mode::Immediate)?, 0);
        Ok(())
    }
}
