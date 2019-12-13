use std::convert::{TryFrom, TryInto};
use thiserror::Error;

pub type Value = i64;

#[derive(Clone, Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid OpCode {0}")]
    InvalidOpCode(Value),
    #[error("Tried to read out of bounds address {0}")]
    SegFault(usize),
}

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

impl TryFrom<Value> for OpCode {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        Ok(match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Halt,
            _ => return Err(Error::InvalidOpCode(value)),
        })
    }
}

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

/*fn get_pointer(&self, address: usize) -> Result<Value> {
    self.get(self.get(address)? as usize)
}*/

pub struct Computer<M: Memory> {
    memory: M,
    ip: usize,
}

impl<M: Memory> Computer<M> {
    pub fn new(memory: M) -> Self {
        Self { ip: 0, memory }
    }

    pub fn execute(&mut self) -> Result<Value> {
        loop {
            let inst: OpCode = self.memory.read(self.ip)?.try_into()?;
            match inst {
                OpCode::Add => self.write(
                    self.memory.read(self.ip + 3)? as usize,
                    self.read(self.ip + 1)? + self.read(self.ip + 2)?,
                )?,
                OpCode::Multiply => self.write(
                    self.memory.read(self.ip + 3)? as usize,
                    self.read(self.ip + 1)? * self.read(self.ip + 2)?,
                )?,
                OpCode::Halt => break,
            };
            self.ip += 4;
        }
        Ok(self.memory.read(0)?)
    }

    fn read(&self, address: usize) -> Result<Value> {
        self.memory.read(self.memory.read(address)? as usize)
    }

    fn write(&mut self, address: usize, value: Value) -> Result<()> {
        self.memory.write(address, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        assert_eq!(OpCode::try_from(1).unwrap(), OpCode::Add);
        assert_eq!(OpCode::try_from(2).unwrap(), OpCode::Multiply);
        assert_eq!(OpCode::try_from(99).unwrap(), OpCode::Halt);
    }

    #[test]
    fn error() {
        assert_eq!(
            OpCode::try_from(55).err().unwrap(),
            Error::InvalidOpCode(55)
        );
    }

    #[test]
    fn test_simple() {
        let mut comp = Computer::new(vec![1, 4, 0, 0, 2, 0, 4, 0, 99]);
        assert_eq!(comp.execute().unwrap(), 6)
    }
}
