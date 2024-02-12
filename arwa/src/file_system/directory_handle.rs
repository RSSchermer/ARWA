use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use js_sys::TypeError;
use pin_project::pin_project;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::DomException;

use crate::console::{Argument, ToArgument};
use crate::file_system::{file_system_handle_seal, FileHandle, FileSystemHandle};
use crate::{impl_common_wrapper_traits, impl_js_cast};

#[derive(Clone)]
pub struct DirectoryHandle {
    inner: web_sys::FileSystemDirectoryHandle,
}

impl DirectoryHandle {
    pub fn get_file_handle(&self, name: &str) -> GetFileHandle {
        GetFileHandle {
            inner: self.inner.get_file_handle(name).into(),
        }
    }

    pub fn get_or_create_file_handle(&self, name: &str) -> GetFileHandle {
        let mut options = web_sys::FileSystemGetFileOptions::new();

        options.create(true);

        GetFileHandle {
            inner: self
                .inner
                .get_file_handle_with_options(name, &options)
                .into(),
        }
    }

    pub fn get_directory_handle(&self, name: &str) -> GetDirectoryHandle {
        GetDirectoryHandle {
            inner: self.inner.get_directory_handle(name).into(),
        }
    }

    pub fn get_or_create_directory_handle(&self, name: &str) -> GetDirectoryHandle {
        let mut options = web_sys::FileSystemGetDirectoryOptions::new();

        options.create(true);

        GetDirectoryHandle {
            inner: self
                .inner
                .get_directory_handle_with_options(name, &options)
                .into(),
        }
    }

    pub fn remove_entry(&self, name: &str) -> RemoveEntry {
        RemoveEntry {
            inner: self.inner.remove_entry(name).into(),
        }
    }

    pub fn remove_entry_recursive(&self, name: &str) -> RemoveEntry {
        let mut options = web_sys::FileSystemRemoveOptions::new();

        options.recursive(true);

        RemoveEntry {
            inner: self.inner.remove_entry_with_options(name, &options).into(),
        }
    }

    // TODO: async iterators over (key, value) pairs and keys and values separately, not currently
    // exposed in web_sys
}

impl file_system_handle_seal::Seal for DirectoryHandle {
    fn as_web_sys(&self) -> &web_sys::FileSystemHandle {
        self.inner.as_ref()
    }
}

impl FileSystemHandle for DirectoryHandle {}

impl_common_wrapper_traits!(DirectoryHandle);
impl_js_cast!(DirectoryHandle, FileSystemDirectoryHandle);

macro_rules! file_system_error {
    ($tpe:ident) => {
        pub struct $tpe {
            inner: JsValue,
        }

        impl $tpe {
            fn new(inner: JsValue) -> Self {
                $tpe { inner }
            }

            pub fn is_invalid_file_name_error(&self) -> bool {
                self.inner.is_instance_of::<TypeError>()
            }

            pub fn is_not_found_error(&self) -> bool {
                if let Some(e) = self.inner.dyn_ref::<DomException>() {
                    e.name() == "NotFoundError"
                } else {
                    false
                }
            }

            pub fn is_not_allowed_error(&self) -> bool {
                if let Some(e) = self.inner.dyn_ref::<DomException>() {
                    e.name() == "NotAllowedError"
                } else {
                    false
                }
            }

            pub fn is_type_mismatch_error(&self) -> bool {
                if let Some(e) = self.inner.dyn_ref::<DomException>() {
                    e.name() == "TypeMismatchError"
                } else {
                    false
                }
            }
        }

        impl From<$tpe> for JsValue {
            fn from(value: $tpe) -> JsValue {
                value.inner
            }
        }

        impl fmt::Display for $tpe {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(type_error) = self.inner.dyn_ref::<TypeError>() {
                    fmt::Display::fmt(&type_error.message(), f)
                } else {
                    // Must be a DOM exception
                    let e = self.inner.unchecked_ref::<DomException>();

                    fmt::Display::fmt(&e.message(), f)
                }
            }
        }

        impl fmt::Debug for $tpe {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }

        impl Error for $tpe {}

        impl ToArgument for $tpe {
            fn to_argument(&self) -> Argument {
                ToArgument::to_argument(&self.inner)
            }
        }
    };
}

file_system_error!(GetFileSystemHandleError);
file_system_error!(RemoveEntryError);

#[pin_project]
pub struct GetFileHandle {
    #[pin]
    inner: JsFuture,
}

impl Future for GetFileHandle {
    type Output = Result<FileHandle, GetFileSystemHandleError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| FileHandle::unchecked_from_js(ok))
            .map_err(|err| GetFileSystemHandleError::new(err))
    }
}

#[pin_project]
pub struct GetDirectoryHandle {
    #[pin]
    inner: JsFuture,
}

impl Future for GetDirectoryHandle {
    type Output = Result<DirectoryHandle, GetFileSystemHandleError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| DirectoryHandle::unchecked_from_js(ok))
            .map_err(|err| GetFileSystemHandleError::new(err))
    }
}

#[pin_project]
pub struct RemoveEntry {
    #[pin]
    inner: JsFuture,
}

impl Future for RemoveEntry {
    type Output = Result<(), RemoveEntryError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| ())
            .map_err(|err| RemoveEntryError::new(err))
    }
}
