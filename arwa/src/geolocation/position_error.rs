use std::error::Error;
use std::fmt;

use crate::console::{Argument, ToArgument};
use crate::normalize_exception_message;
use wasm_bindgen::JsValue;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PositionErrorKind {
    PermissionDenied,
    PositionUnavailable,
    Timeout,
}

#[derive(Clone)]
pub struct PositionError {
    inner: web_sys::PositionError,
}

impl PositionError {
    pub(crate) fn new(inner: web_sys::PositionError) -> Self {
        PositionError { inner }
    }

    pub fn kind(&self) -> PositionErrorKind {
        match self.inner.code() {
            1 => PositionErrorKind::PermissionDenied,
            2 => PositionErrorKind::PositionUnavailable,
            3 => PositionErrorKind::Timeout,
            _ => unreachable!(),
        }
    }
}

impl From<PositionError> for JsValue {
    fn from(value: PositionError) -> Self {
        value.inner.into()
    }
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut message = self.inner.message();

        normalize_exception_message(&mut message);

        fmt::Display::fmt(&message, f)
    }
}

impl fmt::Debug for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl Error for PositionError {}

impl ToArgument for PositionError {
    fn to_argument(&self) -> Argument {
        let js_value: &JsValue = self.inner.as_ref();

        ToArgument::to_argument(js_value)
    }
}
