use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent, UiEvent};

pub struct InputEvent {
    inner: web_sys::InputEvent,
}

impl InputEvent {
    delegate! {
        target self.inner {
            pub fn is_composing(&self) -> bool;
        }
    }
}

impl_common_ui_event_traits!(InputEvent);
