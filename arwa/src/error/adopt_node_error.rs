use crate::error::{HierarchyRequestError, NotSupportedError};

#[derive(Clone, PartialEq, Debug)]
pub enum AdoptNodeError {
    NotSupportedError(NotSupportedError),
    HierarchyRequestError(HierarchyRequestError),
}

impl From<NotSupportedError> for AdoptNodeError {
    fn from(err: NotSupportedError) -> Self {
        AdoptNodeError::NotSupportedError(err)
    }
}

impl From<HierarchyRequestError> for AdoptNodeError {
    fn from(err: HierarchyRequestError) -> Self {
        AdoptNodeError::HierarchyRequestError(err)
    }
}
