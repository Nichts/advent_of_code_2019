use super::errors::{Error, Result};
use super::types::Value;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub(super) enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
}

impl TryFrom<Value> for OpCode {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        Ok(match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            99 => OpCode::Halt,
            _ => return Err(Error::InvalidOpCode(value)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        assert_eq!(OpCode::try_from(1).unwrap(), OpCode::Add);
        assert_eq!(OpCode::try_from(2).unwrap(), OpCode::Multiply);
        assert_eq!(OpCode::try_from(3).unwrap(), OpCode::Input);
        assert_eq!(OpCode::try_from(4).unwrap(), OpCode::Output);
        assert_eq!(OpCode::try_from(99).unwrap(), OpCode::Halt);
    }

    #[test]
    fn error() {
        assert_eq!(
            OpCode::try_from(55).err().unwrap(),
            Error::InvalidOpCode(55)
        );
    }
}
