use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};

pub struct HashChangeEvent {
    inner: web_sys::HashChangeEvent,
}

impl HashChangeEvent {
    delegate! {
        target self.inner {
            pub fn old_url(&self) -> String;

            pub fn new_url(&self) -> String;
        }
    }
}

impl_common_event_traits!(HashChangeEvent);
