use super::mode::Mode;
use super::types::Value;
use thiserror::Error;

#[derive(Clone, Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid OpCode {0}")]
    InvalidOpCode(Value),
    #[error("Tried to read out of bounds address {0}")]
    SegFault(usize),
    #[error("Reading is not supported")]
    ReadingNotSupported,
    #[error("Writing is not supported")]
    WritingNotSupported,
    #[error("Invalid Parameter Mode {0}")]
    InvalidMode(Value),
    #[error("Invalid Write Mode {0:?}")]
    InvalidWriteMode(Mode),
}

pub(super) type Result<T> = ::std::result::Result<T, Error>;
