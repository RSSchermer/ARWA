use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

use crate::stream::{
    readable_stream_seal, writable_stream_seal, ReadableStream, TransformStream, WritableStream,
};

pub struct CompressionStream {
    inner: CompressionStreamInternal,
}

impl CompressionStream {
    pub fn gzip() -> Self {
        CompressionStream {
            inner: CompressionStreamInternal::new("gzip"),
        }
    }

    pub fn deflate() -> Self {
        CompressionStream {
            inner: CompressionStreamInternal::new("deflate"),
        }
    }

    pub fn deflate_raw() -> Self {
        CompressionStream {
            inner: CompressionStreamInternal::new("deflate-raw"),
        }
    }
}

pub struct CompressionWritableStream {
    inner: web_sys::WritableStream,
}

impl writable_stream_seal::Seal for CompressionWritableStream {
    fn as_web_sys(&self) -> &web_sys::WritableStream {
        &self.inner
    }
}

impl WritableStream for CompressionWritableStream {
    type Chunk = Uint8Array;
    type Error = JsValue;
    type Reason = JsValue;
}

pub struct CompressionReadableStream {
    inner: web_sys::ReadableStream,
}

impl readable_stream_seal::Seal for CompressionReadableStream {
    fn as_web_sys(&self) -> &web_sys::ReadableStream {
        &self.inner
    }

    fn from_web_sys(web_sys: web_sys::ReadableStream) -> Self
    where
        Self: Sized,
    {
        CompressionReadableStream { inner: web_sys }
    }
}

impl ReadableStream for CompressionReadableStream {
    type Chunk = Uint8Array;
    type Error = JsValue;
    type Reason = JsValue;
}

impl TransformStream for CompressionStream {
    type In = Uint8Array;

    type Writable = CompressionWritableStream;
    type Readable = CompressionReadableStream;

    fn writable(&self) -> Self::Writable {
        CompressionWritableStream {
            inner: self.inner.writable(),
        }
    }

    fn readable(&self) -> Self::Readable {
        CompressionReadableStream {
            inner: self.inner.readable(),
        }
    }
}

// TODO: no web_sys bindings currently, custom bindings for now, replace later

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = CompressionStream)]
    type CompressionStreamInternal;

    #[wasm_bindgen(constructor)]
    fn new(format: &str) -> CompressionStreamInternal;

    #[wasm_bindgen(method, getter)]
    fn readable(this: &CompressionStreamInternal) -> web_sys::ReadableStream;

    #[wasm_bindgen(method, getter)]
    fn writable(this: &CompressionStreamInternal) -> web_sys::WritableStream;
}
