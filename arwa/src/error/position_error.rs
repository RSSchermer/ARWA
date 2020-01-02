use std::fmt;

#[derive(Clone, PartialEq)]
pub struct PositionError {
    inner: web_sys::PositionError,
}

impl PositionError {
    pub(crate) fn new(inner: web_sys::PositionError) -> Self {
        PositionError { inner }
    }
}

impl fmt::Debug for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.message().fmt(f)
    }
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        String::from(self.inner.message()).fmt(f)
    }
}
