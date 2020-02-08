use crate::console::{Write, Writer};
use crate::error::{SecurityError, SyntaxError};

#[derive(Clone, PartialEq, Debug)]
pub enum LocationAssignError {
    SyntaxError(SyntaxError),
    SecurityError(SecurityError),
}

impl From<SyntaxError> for LocationAssignError {
    fn from(err: SyntaxError) -> Self {
        LocationAssignError::SyntaxError(err)
    }
}

impl From<SecurityError> for LocationAssignError {
    fn from(err: SecurityError) -> Self {
        LocationAssignError::SecurityError(err)
    }
}

impl Write for LocationAssignError {
    fn write(&self, writer: &mut Writer) {
        match self {
            LocationAssignError::SyntaxError(error) => error.write(writer),
            LocationAssignError::SecurityError(error) => error.write(writer),
        }
    }
}
