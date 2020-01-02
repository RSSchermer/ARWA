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
