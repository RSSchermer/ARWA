use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::file_system::DirectoryHandle;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits, impl_js_cast, type_error_wrapper};

pub struct StorageManager {
    inner: web_sys::StorageManager,
}

impl StorageManager {
    pub fn estimate(&self) -> Estimate {
        // No indication the spec that this can throw an exception (though the promise can reject).
        let promise = self.inner.estimate().unwrap_throw();

        Estimate {
            inner: promise.into(),
        }
    }

    pub fn get_directory(&self) -> GetDirectory {
        GetDirectory {
            inner: self.inner.get_directory().into(),
        }
    }
}

impl From<web_sys::StorageManager> for StorageManager {
    fn from(inner: web_sys::StorageManager) -> Self {
        StorageManager { inner }
    }
}

impl_common_wrapper_traits!(StorageManager);
impl_js_cast!(StorageManager);

pub struct StorageEstimates {
    inner: JsValue,
}

impl StorageEstimates {
    fn new(inner: JsValue) -> Self {
        StorageEstimates { inner }
    }

    pub fn quota(&self) -> u64 {
        js_sys::Reflect::get(&self.inner, &JsValue::from_str("quota"))
            .unwrap_throw()
            .as_f64()
            .unwrap_throw() as u64
    }

    pub fn usage(&self) -> u64 {
        js_sys::Reflect::get(&self.inner, &JsValue::from_str("usage"))
            .unwrap_throw()
            .as_f64()
            .unwrap_throw() as u64
    }
}

type_error_wrapper!(EstimateError);

#[pin_project]
pub struct Estimate {
    #[pin]
    inner: JsFuture,
}

impl Future for Estimate {
    type Output = Result<StorageEstimates, EstimateError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| StorageEstimates::new(ok))
            .map_err(|err| EstimateError::new(err.unchecked_into()))
    }
}

dom_exception_wrapper!(GetDirectoryError);

#[pin_project]
pub struct GetDirectory {
    #[pin]
    inner: JsFuture,
}

impl Future for GetDirectory {
    type Output = Result<DirectoryHandle, GetDirectoryError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| DirectoryHandle::unchecked_from_js(ok))
            .map_err(|err| GetDirectoryError::new(err.unchecked_into()))
    }
}
