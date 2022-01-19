use crate::security::SecurityError;

#[derive(Clone)]
pub enum CreateWorkerError {
    SecurityError(SecurityError),
    NetworkError(CreateWorkerNetworkError),
}

impl CreateWorkerError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        if inner.code() == 18 {
            CreateWorkerError::SecurityError(SecurityError::new(inner))
        } else {
            CreateWorkerError::NetworkError(CreateWorkerNetworkError::new(inner))
        }
    }
}

#[derive(Clone)]
pub struct CreateWorkerNetworkError {
    inner: web_sys::DomException,
}

impl CreateWorkerNetworkError {
    fn new(inner: web_sys::DomException) -> Self {
        CreateWorkerNetworkError { inner }
    }
}
