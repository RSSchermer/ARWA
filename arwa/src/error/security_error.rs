use crate::console::{Write, Writer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct SecurityError {
    inner: web_sys::DomException,
}

impl SecurityError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        SecurityError { inner }
    }
}

impl fmt::Debug for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Security error: {}", self.inner.message())
    }
}

impl Write for SecurityError {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl AsRef<web_sys::DomException> for SecurityError {
    fn as_ref(&self) -> &web_sys::DomException {
        &self.inner
    }
}
