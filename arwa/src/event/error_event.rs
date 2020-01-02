use delegate::delegate;
use wasm_bindgen::{JsCast, JsValue};

use crate::event::{Event, FromEvent};

pub struct ErrorEvent {
    inner: web_sys::ErrorEvent,
}

impl ErrorEvent {
    delegate! {
        target self.inner {
            pub fn message(&self) -> String;

            pub fn filename(&self) -> String;

            pub fn error(&self) -> JsValue;
        }
    }

    pub fn line_number(&self) -> u32 {
        self.inner.lineno()
    }

    pub fn column_number(&self) -> u32 {
        self.inner.colno()
    }
}

impl_common_event_traits!(ErrorEvent);
