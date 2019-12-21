use delegate::delegate;
use wasm_bindgen::{JsCast, JsValue};

use crate::event::{Event, FromEvent};

pub struct MessageEvent {
    inner: web_sys::MessageEvent,
}

impl MessageEvent {
    // TODO: `source` and `ports`. Defer to the plain js_sys objects or add wrappers here?

    delegate! {
        target self.inner {
            pub fn data(&self) -> JsValue;

            pub fn origin(&self) -> String;

            pub fn last_event_id(&self) -> String;
        }
    }
}

impl_common_event_traits!(MessageEvent);
