use crate::console::{Write, Writer};
use crate::error::{InvalidStateError, RangeError};

#[derive(Clone, PartialEq, Debug)]
pub enum SetTextRangeError {
    InvalidState(InvalidStateError),
    InvalidRange(RangeError),
}

impl From<InvalidStateError> for SetTextRangeError {
    fn from(err: InvalidStateError) -> Self {
        SetTextRangeError::InvalidState(err)
    }
}

impl From<RangeError> for SetTextRangeError {
    fn from(err: RangeError) -> Self {
        SetTextRangeError::InvalidRange(err)
    }
}

impl Write for SetTextRangeError {
    fn write(&self, writer: &mut Writer) {
        match self {
            SetTextRangeError::InvalidState(error) => error.write(writer),
            SetTextRangeError::InvalidRange(error) => error.write(writer),
        }
    }
}
