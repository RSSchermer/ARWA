use std::fmt;

#[derive(Clone, PartialEq)]
pub struct InvalidStateError {
    inner: web_sys::DomException,
}

impl InvalidStateError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        InvalidStateError { inner }
    }
}

impl fmt::Debug for InvalidStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Invalid state error: {}", self.inner.message())
    }
}

impl AsRef<web_sys::DomException> for InvalidStateError {
    fn as_ref(&self) -> &web_sys::DomException {
        &self.inner
    }
}
