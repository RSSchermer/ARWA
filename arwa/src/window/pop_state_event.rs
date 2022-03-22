use std::marker;

use delegate::delegate;
use wasm_bindgen::JsValue;

use crate::event::impl_typed_event_traits;

#[derive(Clone)]
pub struct PopStateEvent<T> {
    inner: web_sys::PopStateEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> PopStateEvent<T> {
    delegate! {
        to self.inner {
            pub fn state(&self) -> JsValue;
        }
    }
}

impl_typed_event_traits!(PopStateEvent, PopStateEvent, "popstate");
