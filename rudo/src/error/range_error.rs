use std::fmt;

#[derive(Clone, PartialEq)]
pub struct RangeError {
    inner: web_sys::DomException,
}

impl RangeError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        RangeError { inner }
    }
}

impl fmt::Debug for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Range error: {}", self.inner.message())
    }
}

impl AsRef<web_sys::DomException> for RangeError {
    fn as_ref(&self) -> &web_sys::DomException {
        &self.inner
    }
}
