use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};

pub struct AnimationEvent {
    inner: web_sys::AnimationEvent,
}

impl AnimationEvent {
    delegate! {
        target self.inner {
            pub fn animation_name(&self) -> String;

            pub fn elapsed_time(&self) -> f32;
        }
    }
}

impl_common_event_traits!(AnimationEvent);
