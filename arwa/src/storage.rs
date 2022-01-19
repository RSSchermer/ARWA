use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::console::{Write, Writer};
use crate::error::QuotaExceededError;

#[derive(Clone)]
pub struct Storage {
    inner: web_sys::Storage,
}

impl Storage {
    pub fn key_at(&self, index: u32) -> Option<String> {
        self.inner.key(index).unwrap_throw()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.get_item(key).unwrap_throw()
    }

    pub fn set(&self, key: &str, value: &str) {
        self.inner.set_item(key, value).unwrap_throw();
    }

    pub fn try_set(&self, key: &str, value: &str) -> Result<(), QuotaExceededError> {
        self.inner.set_item(key, value).map_err(|err| {
            let err: web_sys::DomException = err.unchecked_into();

            QuotaExceededError::new(err)
        })
    }

    pub fn remove(&self, key: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.remove_item(key).unwrap_throw();
    }

    pub fn clear(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.clear().unwrap_throw();
    }
}

impl Collection for Storage {
    fn len(&self) -> u32 {
        self.inner.length().unwrap_throw()
    }
}

impl From<web_sys::Storage> for Storage {
    fn from(inner: web_sys::Storage) -> Self {
        Storage { inner }
    }
}

impl AsRef<web_sys::Storage> for Storage {
    fn as_ref(&self) -> &web_sys::Storage {
        &self.inner
    }
}

impl_common_wrapper_traits!(Storage);
