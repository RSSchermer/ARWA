use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent};

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

            // TODO: wrap Storage API in this crate?
            pub fn storage_area(&self) -> Option<web_sys::Storage>;
        }
    }
}

impl_common_event_traits!(StorageEvent);
