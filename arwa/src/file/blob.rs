use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(Clone)]
pub struct Blob {
    inner: web_sys::Blob,
}

impl Blob {
    pub fn from_bytes(bytes: &[u8], format_type: &str) -> Self {
        unsafe {
            let array_buffer = js_sys::Uint8Array::view(bytes);
            let mut options = web_sys::BlobPropertyBag::new();

            options.type_(format_type);

            Blob {
                inner: web_sys::Blob::new_with_buffer_source_sequence_and_options(
                    array_buffer.into(),
                    &options,
                ),
            }
        }
    }

    pub fn view(blob: &Blob, format_type: &str) -> Self {
        Blob {
            inner: blob.inner.slice_with_f64_and_f64_and_content_type(
                0,
                blob.inner.size(),
                format_type,
            ),
        }
    }

    pub fn format_type(&self) -> String {
        self.inner.type_()
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
                    .slice_with_f64_and_f64(self.start() as f64, (self.end() + 1) as f64)
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
