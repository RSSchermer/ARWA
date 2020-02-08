use crate::console::{Write, Writer};
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

impl Write for RegisterProtocolHandlerError {
    fn write(&self, writer: &mut Writer) {
        match self {
            RegisterProtocolHandlerError::SecurityError(error) => error.write(writer),
            RegisterProtocolHandlerError::SyntaxError(error) => error.write(writer),
        }
    }
}
