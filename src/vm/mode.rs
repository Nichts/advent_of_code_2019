use super::errors::{Error, Result};
use super::types::Value;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<Value> for Mode {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            val => Err(Error::InvalidMode(val)),
        }
    }
}
