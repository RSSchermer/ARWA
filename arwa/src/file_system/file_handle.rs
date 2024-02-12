use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use js_sys::Uint8Array;
use pin_project::pin_project;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::file::{Blob, File};
use crate::file_system::{file_system_handle_seal, FileSystemHandle};
use crate::stream::{writable_stream_seal, WritableStream};
use crate::{dom_exception_wrapper, impl_common_wrapper_traits, impl_js_cast};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WritableExistingData {
    Keep,
    Discard,
}

impl Default for WritableExistingData {
    fn default() -> Self {
        WritableExistingData::Discard
    }
}

#[derive(Clone)]
pub struct FileHandle {
    inner: web_sys::FileSystemFileHandle,
}

impl FileHandle {
    pub fn get_file(&self) -> GetFile {
        GetFile {
            inner: self.inner.get_file().into(),
        }
    }

    pub fn create_writable(&self, existing_data: WritableExistingData) -> CreateWritable {
        match existing_data {
            WritableExistingData::Keep => {
                let mut options = web_sys::FileSystemCreateWritableOptions::new();

                options.keep_existing_data(true);

                CreateWritable {
                    inner: self.inner.create_writable_with_options(&options).into(),
                }
            }
            WritableExistingData::Discard => CreateWritable {
                inner: self.inner.create_writable().into(),
            },
        }
    }
}

impl file_system_handle_seal::Seal for FileHandle {
    fn as_web_sys(&self) -> &web_sys::FileSystemHandle {
        self.inner.as_ref()
    }
}

impl FileSystemHandle for FileHandle {}

impl_common_wrapper_traits!(FileHandle);
impl_js_cast!(FileHandle, FileSystemFileHandle);

dom_exception_wrapper!(GetFileError);

impl GetFileError {
    pub fn is_not_found_error(&self) -> bool {
        self.inner.name() == "NotFoundError"
    }

    pub fn is_not_allowed_error(&self) -> bool {
        self.inner.name() == "NotAllowedError"
    }
}

#[pin_project]
pub struct GetFile {
    #[pin]
    inner: JsFuture,
}

impl Future for GetFile {
    type Output = Result<File, GetFileError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| File::unchecked_from_js(ok))
            .map_err(|err| GetFileError::new(err.unchecked_into()))
    }
}

dom_exception_wrapper!(CreateWritableError);

impl CreateWritableError {
    pub fn is_not_found_error(&self) -> bool {
        self.inner.name() == "NotFoundError"
    }

    pub fn is_not_allowed_error(&self) -> bool {
        self.inner.name() == "NotAllowedError"
    }

    pub fn is_no_modification_allowed_error(&self) -> bool {
        self.inner.name() == "NoModificationAllowedError"
    }

    pub fn is_abort_error(&self) -> bool {
        self.inner.name() == "AbortError"
    }
}

#[pin_project]
pub struct CreateWritable {
    #[pin]
    inner: JsFuture,
}

impl Future for CreateWritable {
    type Output = Result<FileWritableStream, CreateWritableError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| FileWritableStream {
                inner: ok.unchecked_into(),
            })
            .map_err(|err| CreateWritableError::new(err.unchecked_into()))
    }
}

pub struct FileWritableStream {
    inner: web_sys::FileSystemWritableFileStream,
}

impl FileWritableStream {
    pub fn write_bytes(&self, bytes: &[u8]) -> FileWrite {
        // Spec does not indicate this can return an error (but the promise can resolve with an error)
        let promise = self.inner.write_with_u8_array(bytes).unwrap_throw();

        FileWrite {
            inner: promise.into(),
        }
    }

    pub fn write_str(&self, string: &str) -> FileWrite {
        // Spec does not indicate this can return an error (but the promise can resolve with an error)
        let promise = self.inner.write_with_str(string).unwrap_throw();

        FileWrite {
            inner: promise.into(),
        }
    }

    pub fn write_blob(&self, blob: Blob) -> FileWrite {
        // Spec does not indicate this can return an error (but the promise can resolve with an error)
        let promise = self.inner.write_with_blob(blob.as_ref()).unwrap_throw();

        FileWrite {
            inner: promise.into(),
        }
    }

    pub fn seek(&self, position: u64) -> FileSeek {
        // Spec does not indicate this can return an error (but the promise can resolve with an error)
        let promise = self.inner.seek_with_f64(position as f64).unwrap_throw();

        FileSeek {
            inner: promise.into(),
        }
    }

    pub fn truncate(&self, size: u64) -> FileWrite {
        // Spec does not indicate this can return an error (but the promise can resolve with an error)
        let promise = self.inner.truncate_with_f64(size as f64).unwrap_throw();

        FileWrite {
            inner: promise.into(),
        }
    }
}

impl writable_stream_seal::Seal for FileWritableStream {
    fn as_web_sys(&self) -> &web_sys::WritableStream {
        self.inner.as_ref()
    }
}

impl WritableStream for FileWritableStream {
    type Chunk = Uint8Array;
    type Error = JsValue;
    type Reason = JsValue;
}

dom_exception_wrapper!(FileWriteError);

impl FileWriteError {
    pub fn is_not_allowed_error(&self) -> bool {
        self.inner.name() == "NotAllowedError"
    }

    pub fn is_quota_exceeded_error(&self) -> bool {
        self.inner.name() == "QuotaExceededError"
    }
}

#[pin_project]
pub struct FileWrite {
    #[pin]
    inner: JsFuture,
}

impl Future for FileWrite {
    type Output = Result<(), FileWriteError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| ())
            .map_err(|err| FileWriteError::new(err.unchecked_into()))
    }
}

dom_exception_wrapper!(FileSeekError);

impl FileSeekError {
    pub fn is_not_allowed_error(&self) -> bool {
        self.inner.name() == "NotAllowedError"
    }
}

#[pin_project]
pub struct FileSeek {
    #[pin]
    inner: JsFuture,
}

impl Future for FileSeek {
    type Output = Result<(), FileSeekError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| ())
            .map_err(|err| FileSeekError::new(err.unchecked_into()))
    }
}
