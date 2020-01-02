use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};

pub struct ProgressEvent {
    inner: web_sys::ProgressEvent,
}

impl ProgressEvent {
    delegate! {
        target self.inner {
            pub fn length_computable(&self) -> bool;

            pub fn loaded(&self) -> f64;

            pub fn total(&self) -> f64;
        }
    }
}

impl_common_event_traits!(ProgressEvent);
