use std::marker;

use delegate::delegate;
use wasm_bindgen::UnwrapThrowExt;

use crate::event::impl_typed_event_traits;
use crate::storage::Storage;
use crate::url::Url;

#[derive(Clone)]
pub struct StorageEvent<T> {
    inner: web_sys::StorageEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> StorageEvent<T> {
    delegate! {
        target self.inner {
            pub fn key(&self) -> Option<String>;

            pub fn old_value(&self) -> Option<String>;

            pub fn new_value(&self) -> Option<String>;
        }
    }

    pub fn storage_area(&self) -> Option<Storage> {
        self.inner.storage_area().map(|s| s.into())
    }

    pub fn url(&self) -> Option<Url> {
        self.inner
            .url()
            .map(|s| Url::parse(s.as_ref()).unwrap_throw())
    }
}

impl_typed_event_traits!(StorageEvent, StorageEvent, "storage");
