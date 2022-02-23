use std::mem;
use std::ops::Deref;

use wasm_bindgen::UnwrapThrowExt;

use crate::file::Blob;
use crate::impl_common_wrapper_traits;

#[derive(Clone)]
pub struct File {
    inner: web_sys::File,
}

impl File {
    pub fn new(data: Blob, name: &str) -> Self {
        File {
            inner: web_sys::File::new_with_blob_sequence(data.as_ref(), name).unwrap_throw(),
        }
    }

    pub fn new_with_last_modified(data: Blob, name: &str, last_modified: u64) -> Self {
        let mut options = web_sys::FilePropertyBag::new();

        options.last_modified(last_modified as f64);

        File {
            inner: web_sys::File::new_with_blob_sequence_and_options(data.as_ref(), name, &options)
                .unwrap_throw(),
        }
    }

    pub fn name(&self) -> String {
        self.inner.name()
    }

    pub fn last_modified(&self) -> u64 {
        self.inner.last_modified() as u64
    }
}

impl Deref for File {
    type Target = Blob;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Only safe as long as Blob is declared as `Blob { inner: web_sys::Blob }`
            mem::transmute(self)
        }
    }
}

impl AsRef<web_sys::File> for File {
    fn as_ref(&self) -> &web_sys::File {
        &self.inner
    }
}

impl From<web_sys::File> for File {
    fn from(inner: web_sys::File) -> Self {
        File { inner }
    }
}

impl_common_wrapper_traits!(File);
