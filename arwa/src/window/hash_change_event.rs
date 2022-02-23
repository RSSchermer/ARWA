use std::marker;

use wasm_bindgen::UnwrapThrowExt;

use crate::event::impl_typed_event_traits;
use crate::url::Url;

#[derive(Clone)]
pub struct HashChangeEvent<T> {
    inner: web_sys::HashChangeEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> HashChangeEvent<T> {
    pub fn old_url(&self) -> Url {
        Url::parse(self.inner.old_url().as_ref()).unwrap_throw()
    }

    pub fn new_url(&self) -> Url {
        Url::parse(self.inner.new_url().as_ref()).unwrap_throw()
    }
}

impl_typed_event_traits!(HashChangeEvent, HashChangeEvent, "hashchange");
