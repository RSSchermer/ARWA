use crate::console::{Write, Writer};
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct HierarchyRequestError {
    inner: web_sys::DomException,
}

impl HierarchyRequestError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        HierarchyRequestError { inner }
    }
}

impl fmt::Debug for HierarchyRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Hierarchy request error: {}", self.inner.message())
    }
}

impl Write for HierarchyRequestError {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl AsRef<web_sys::DomException> for HierarchyRequestError {
    fn as_ref(&self) -> &web_sys::DomException {
        &self.inner
    }
}
