use crate::storage::Storage;
use std::marker;
use url::Url;

#[derive(Clone)]
pub struct StorageEvent<T> {
    inner: web_sys::StorageEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> StorageEvent<T> {
    delegate! {
        to self.inner {
            pub fn key(&self) -> Option<String>;

            pub fn old_value(&self) -> Option<String>;

            pub fn new_value(&self) -> Option<String>;
        }
    }

    pub fn storage_area(&self) -> Option<Storage> {
        self.inner.storage_area().map(|s| s.into())
    }

    pub fn url(&self) -> Option<Url> {
        self.inner.url().map(|s| Url::parse(s.as_ref()).unwrap())
    }
}

impl_event_traits!(StorageEvent, web_sys::StorageEvent, "storage");
