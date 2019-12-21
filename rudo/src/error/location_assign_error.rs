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
