pub mod execution;

use crate::consts::msg;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("{}", msg::SAMPLE_ERROR)]
    SampleError,
    #[error(transparent)]
    InstructionDecodeError(#[from] execution::Error),
}
