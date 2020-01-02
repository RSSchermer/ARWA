use std::fmt;

#[derive(Clone, PartialEq)]
pub struct TypeError {
    inner: js_sys::TypeError,
}

impl TypeError {
    pub(crate) fn new(inner: js_sys::TypeError) -> Self {
        TypeError { inner }
    }
}

impl fmt::Debug for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.message().fmt(f)
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        String::from(self.inner.message()).fmt(f)
    }
}
