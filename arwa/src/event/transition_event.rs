use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};

pub struct TransitionEvent {
    inner: web_sys::TransitionEvent,
}

impl TransitionEvent {
    delegate! {
        target self.inner {
            pub fn property_name(&self) -> String;

            pub fn elapsed_time(&self) -> f32;
        }
    }
}

impl_common_event_traits!(TransitionEvent);
