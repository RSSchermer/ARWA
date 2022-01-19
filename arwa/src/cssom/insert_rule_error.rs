#[derive(Clone)]
pub struct InsertRuleError {
    inner: web_sys::DomException,
}

impl InsertRuleError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        InsertRuleError { inner }
    }
}
