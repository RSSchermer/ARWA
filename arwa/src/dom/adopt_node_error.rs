use crate::console::{Write, Writer};
use crate::error::{HierarchyRequestError, NotSupportedError};

#[derive(Clone)]
pub struct AdoptNodeError {
    inner: web_sys::DomException,
}

impl AdoptNodeError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        AdoptNodeError { inner }
    }
}
