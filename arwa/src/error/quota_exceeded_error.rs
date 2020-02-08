use crate::console::{Write, Writer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct QuotaExceededError {
    inner: web_sys::DomException,
}

impl QuotaExceededError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        QuotaExceededError { inner }
    }
}

impl fmt::Debug for QuotaExceededError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Quota exceeded error: {}", self.inner.message())
    }
}

impl Write for QuotaExceededError {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl AsRef<web_sys::DomException> for QuotaExceededError {
    fn as_ref(&self) -> &web_sys::DomException {
        &self.inner
    }
}
