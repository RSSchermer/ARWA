use crate::error::{SecurityError, SyntaxError};

#[derive(Clone, PartialEq, Debug)]
pub enum RegisterProtocolHandlerError {
    SecurityError(SecurityError),
    SyntaxError(SyntaxError),
}

impl From<SecurityError> for RegisterProtocolHandlerError {
    fn from(err: SecurityError) -> Self {
        RegisterProtocolHandlerError::SecurityError(err)
    }
}

impl From<SyntaxError> for RegisterProtocolHandlerError {
    fn from(err: SyntaxError) -> Self {
        RegisterProtocolHandlerError::SyntaxError(err)
    }
}
