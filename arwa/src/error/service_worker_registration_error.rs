use std::fmt;

#[derive(Clone, PartialEq)]
pub struct ServiceWorkerRegistrationError {
    inner: js_sys::Error,
}

impl ServiceWorkerRegistrationError {
    pub(crate) fn new(inner: js_sys::Error) -> Self {
        ServiceWorkerRegistrationError { inner }
    }
}

impl fmt::Debug for ServiceWorkerRegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.message().fmt(f)
    }
}

impl fmt::Display for ServiceWorkerRegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        String::from(self.inner.message()).fmt(f)
    }
}
