use crate::dom_exception_wrapper;

dom_exception_wrapper!(CreateWorkerError);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CreateWorkerErrorKind {
    Security,
    Network,
}

impl CreateWorkerError {
    pub fn kind(&self) -> CreateWorkerErrorKind {
        match self.inner.name().as_str() {
            "SecurityError" => CreateWorkerErrorKind::Security,
            "NetworkError" => CreateWorkerErrorKind::Network,
            _ => unreachable!(),
        }
    }
}
