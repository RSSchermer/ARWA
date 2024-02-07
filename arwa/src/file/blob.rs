use std::future::Future;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::pin::Pin;
use std::task::{Context, Poll};

use js_sys::{ArrayBuffer, Uint8Array};
use pin_project::pin_project;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::media_type::MediaType;
use crate::stream::{readable_stream_seal, ReadableStream};
use crate::{impl_common_wrapper_traits, impl_js_cast, type_error_wrapper};

#[derive(Clone)]
pub struct Blob {
    inner: web_sys::Blob,
}

impl Blob {
    pub fn from_bytes(bytes: &[u8], media_type: Option<&MediaType>) -> Self {
        unsafe {
            let array_buffer = js_sys::Uint8Array::view(bytes);
            let mut options = web_sys::BlobPropertyBag::new();

            if let Some(media_type) = media_type {
                options.type_(media_type.as_ref());
            }

            // Note for the version of the Blob constructor that takes a buffer source, there is no
            // indication in the spec that it can fail, so we unwrap.
            Blob {
                inner: web_sys::Blob::new_with_buffer_source_sequence_and_options(
                    &array_buffer.into(),
                    &options,
                )
                .unwrap_throw(),
            }
        }
    }

    pub fn view(blob: &Blob, media_type: Option<&MediaType>) -> Self {
        // No indication in the spec that this can fail, unwrap.
        Blob {
            inner: blob
                .inner
                .slice_with_i32_and_f64_and_content_type(
                    0,
                    blob.inner.size(),
                    media_type.map(|m| m.as_ref()).unwrap_or(""),
                )
                .unwrap_throw(),
        }
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    // Note: std seems to prefer `len` to refer to a file size in bytes (e.g.
    // `std::fs::File::set_len`), conforming to that convention.

    pub fn len(&self) -> u64 {
        self.inner.size() as u64
    }

    pub fn get<R>(&self, range: R) -> Option<Blob>
    where
        R: BlobRange,
    {
        range.get(self)
    }

    pub fn get_array_buffer(&self) -> GetArrayBuffer {
        GetArrayBuffer {
            inner: self.inner.array_buffer().into(),
        }
    }

    pub fn to_readable_stream(&self) -> BlobReadableStream {
        BlobReadableStream {
            inner: self.inner.stream(),
        }
    }

    // TODO: read interface. Probably something involving piping the blob's ReadableStream into a
    // custom WriteableStream that writes directly into Rust controller memory
    // (a std::io::BufReader?). Simply calling `arrayBuffer` to create a copy and then copying again
    // into rust memory does not seem appropriate.
    //
    // Also, only expose a binary (byte data) interface here, no text decoding? Using a WASM-based
    // decoder is probably not significantly slower than using the browser's decoding machinery and
    // keeps the interface lean.
}

impl_common_wrapper_traits!(Blob);
impl_js_cast!(Blob);

impl From<web_sys::Blob> for Blob {
    fn from(inner: web_sys::Blob) -> Self {
        Blob { inner }
    }
}

impl AsRef<web_sys::Blob> for Blob {
    fn as_ref(&self) -> &web_sys::Blob {
        &self.inner
    }
}

// Note: going with u64 for for slice indexing here, as use-cases where u32 is too small seem very
// realistic for files/binary blobs. However, that currently means casting to f64, which cannot
// accurately represent all u64; it would start to break at 2^53, but then, that's probably not
// a realistically valid number for a blob/file size. Opting to simply cast without a check for
// now and hope that as WASM evolves, we'll eventually get proper u64 support.

mod blob_range_seal {
    use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

    pub trait Seal {}

    impl Seal for RangeFull {}
    impl Seal for Range<u64> {}
    impl Seal for RangeInclusive<u64> {}
    impl Seal for RangeFrom<u64> {}
    impl Seal for RangeTo<u64> {}
    impl Seal for RangeToInclusive<u64> {}
}

pub trait BlobRange: blob_range_seal::Seal {
    fn get(self, blob: &Blob) -> Option<Blob>;
}

impl BlobRange for RangeFull {
    fn get(self, blob: &Blob) -> Option<Blob> {
        Some(blob.clone())
    }
}

impl BlobRange for Range<u64> {
    fn get(self, blob: &Blob) -> Option<Blob> {
        if !self.is_empty() && self.end <= blob.len() {
            Some(Blob {
                inner: blob
                    .inner
                    .slice_with_f64_and_f64(self.start as f64, self.end as f64)
                    .unwrap_throw(),
            })
        } else {
            None
        }
    }
}

impl BlobRange for RangeInclusive<u64> {
    fn get(self, blob: &Blob) -> Option<Blob> {
        if !self.is_empty() && self.end() < &blob.len() {
            Some(Blob {
                inner: blob
                    .inner
                    .slice_with_f64_and_f64(*self.start() as f64, (*self.end() + 1) as f64)
                    .unwrap_throw(),
            })
        } else {
            None
        }
    }
}

impl BlobRange for RangeFrom<u64> {
    fn get(self, blob: &Blob) -> Option<Blob> {
        let len = blob.len();

        if self.start < len {
            Some(Blob {
                inner: blob
                    .inner
                    .slice_with_f64_and_f64(self.start as f64, len as f64)
                    .unwrap_throw(),
            })
        } else {
            None
        }
    }
}

impl BlobRange for RangeTo<u64> {
    fn get(self, blob: &Blob) -> Option<Blob> {
        if self.end <= blob.len() {
            Some(Blob {
                inner: blob
                    .inner
                    .slice_with_f64_and_f64(0.0, self.end as f64)
                    .unwrap_throw(),
            })
        } else {
            None
        }
    }
}

impl BlobRange for RangeToInclusive<u64> {
    fn get(self, blob: &Blob) -> Option<Blob> {
        if self.end < blob.len() {
            Some(Blob {
                inner: blob
                    .inner
                    .slice_with_f64_and_f64(0.0, (self.end + 1) as f64)
                    .unwrap_throw(),
            })
        } else {
            None
        }
    }
}

pub struct BlobReadableStream {
    inner: web_sys::ReadableStream,
}

impl readable_stream_seal::Seal for BlobReadableStream {
    fn as_web_sys(&self) -> &web_sys::ReadableStream {
        &self.inner
    }

    fn from_web_sys(inner: web_sys::ReadableStream) -> Self
    where
        Self: Sized,
    {
        BlobReadableStream { inner }
    }
}

impl ReadableStream for BlobReadableStream {
    type Chunk = Uint8Array;
    type Error = JsValue;
    type Reason = JsValue;
}

type_error_wrapper!(GetArrayBufferError);

#[pin_project]
pub struct GetArrayBuffer {
    #[pin]
    inner: JsFuture,
}

impl Future for GetArrayBuffer {
    type Output = Result<ArrayBuffer, GetArrayBufferError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map_ok(|ok| ok.unchecked_into())
            .map_err(|err| GetArrayBufferError::new(err.unchecked_into()))
    }
}
