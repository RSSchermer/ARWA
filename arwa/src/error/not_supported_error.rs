use crate::console::{Write, Writer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct NotSupportedError {
    inner: web_sys::DomException,
}

impl NotSupportedError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        NotSupportedError { inner }
    }
}

impl fmt::Debug for NotSupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Not supported error: {}", self.inner.message())
    }
}

impl Write for NotSupportedError {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl AsRef<web_sys::DomException> for NotSupportedError {
    fn as_ref(&self) -> &web_sys::DomException {
        &self.inner
    }
}
