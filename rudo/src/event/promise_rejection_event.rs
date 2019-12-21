use delegate::delegate;
use js_sys::Promise;
use wasm_bindgen::{JsCast, JsValue};

use crate::event::{Event, FromEvent};

pub struct PromiseRejectionEvent {
    inner: web_sys::PromiseRejectionEvent,
}

impl PromiseRejectionEvent {
    delegate! {
        target self.inner {
            pub fn promise(&self) -> Promise;

            pub fn reason(&self) -> JsValue;
        }
    }
}

impl_common_event_traits!(PromiseRejectionEvent);
