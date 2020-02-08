use crate::console::{Write, Writer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct SyntaxError {
    inner: js_sys::SyntaxError,
}

impl SyntaxError {
    pub(crate) fn new(inner: js_sys::SyntaxError) -> Self {
        SyntaxError { inner }
    }
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.message().fmt(f)
    }
}

impl Write for SyntaxError {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        String::from(self.inner.message()).fmt(f)
    }
}
