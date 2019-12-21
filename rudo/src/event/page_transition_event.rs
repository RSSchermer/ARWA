use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};

pub struct PageTransitionEvent {
    inner: web_sys::PageTransitionEvent,
}

impl PageTransitionEvent {
    delegate! {
        target self.inner {
            pub fn persisted(&self) -> bool;
        }
    }
}

impl_common_event_traits!(PageTransitionEvent);
