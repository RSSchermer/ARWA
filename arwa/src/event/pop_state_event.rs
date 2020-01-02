use delegate::delegate;
use wasm_bindgen::{JsCast, JsValue};

use crate::event::{Event, FromEvent};

pub struct PopStateEvent {
    inner: web_sys::PopStateEvent,
}

impl PopStateEvent {
    delegate! {
        target self.inner {
            pub fn state(&self) -> JsValue;
        }
    }
}

impl_common_event_traits!(PopStateEvent);
