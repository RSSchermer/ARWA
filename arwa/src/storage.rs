use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::error::QuotaExceededError;

pub struct Storage {
    inner: web_sys::Storage,
}

impl Storage {
    pub fn get(&self, key: &str) -> Option<String> {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.get_item(key).unwrap()
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), QuotaExceededError> {
        self.inner.set_item(key, value).map_err(|err| {
            let err: web_sys::DomException = err.unchecked_into();

            QuotaExceededError::new(err)
        })
    }

    pub fn remove(&self, key: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.remove_item(key).unwrap();
    }

    pub fn clear(&self) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.clear().unwrap();
    }

    pub fn len(&self) -> usize {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.length().unwrap() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn keys(&self) -> StorageKeys {
        StorageKeys { storage: self }
    }

    // TODO: its possible to mimic HashMap's (key, value) iter style and `values` collection, but
    // I'm not sure if this is appropriate
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

#[derive(Clone, Copy)]
pub struct StorageKeys<'a> {
    storage: &'a Storage,
}

impl<'a> StorageKeys<'a> {
    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index).ok().and_then(|index| {
            // No indication in the spec that this can fail, unwrap for now.
            self.storage.inner.key(index).unwrap()
        })
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<String> {
        self.get(0)
    }

    pub fn last(&self) -> Option<String> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> StorageKeysIter {
        StorageKeysIter {
            storage_keys: *self,
            current: 0,
        }
    }
}

impl<'a> IntoIterator for StorageKeys<'a> {
    type Item = String;
    type IntoIter = StorageKeysIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StorageKeysIter {
            storage_keys: self,
            current: 0,
        }
    }
}

pub struct StorageKeysIter<'a> {
    storage_keys: StorageKeys<'a>,
    current: usize,
}

impl<'a> Iterator for StorageKeysIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.storage_keys.get(current)
    }
}
