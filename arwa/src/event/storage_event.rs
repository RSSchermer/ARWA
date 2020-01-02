use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};
use crate::Storage;

pub struct StorageEvent {
    inner: web_sys::StorageEvent,
}

impl StorageEvent {
    delegate! {
        target self.inner {
            pub fn key(&self) -> Option<String>;

            pub fn old_value(&self) -> Option<String>;

            pub fn new_value(&self) -> Option<String>;

            pub fn url(&self) -> Option<String>;
        }
    }

    pub fn storage_area(&self) -> Option<Storage> {
        self.inner.storage_area().map(|s| s.into())
    }
}

impl_common_event_traits!(StorageEvent);
